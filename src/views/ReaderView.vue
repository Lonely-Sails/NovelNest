<template>
  <div class="reader" v-if="book" :class="{ 'fullscreen': isFullscreen }">
    <!-- 阅读器头部 -->
    <div class="reader-header" v-show="!isFullscreen || showControls">
      <button @click="$router.back()" class="back-btn">
        <span class="icon">←</span>
        <span>返回</span>
      </button>
      
      <div class="book-info">
        <h2>{{ book.title }}</h2>
        <span class="author">{{ book.author || '未知作者' }}</span>
        <span class="chapter-info" v-if="currentChapter">
          {{ currentChapter.title }}
        </span>
      </div>
      
      <div class="reader-controls">
        <button @click="toggleSearch" class="control-btn" title="搜索 (Ctrl+F)">
          <span class="icon">🔍</span>
        </button>
        <button @click="toggleBookmarks" class="control-btn" title="书签 (B)">
          <span class="icon">📌</span>
        </button>
        <button @click="toggleFullscreen" class="control-btn" title="全屏 (F11)">
          <span class="icon">{{ isFullscreen ? '🗗' : '🗖' }}</span>
        </button>
        <button @click="toggleSettings" class="control-btn" title="设置 (S)">
          <span class="icon">⚙️</span>
        </button>
        <button @click="toggleToc" class="control-btn" title="目录 (T)">
          <span class="icon">📋</span>
        </button>
      </div>
    </div>

    <!-- 主要阅读区域 -->
    <div class="reader-content" @click="handleContentClick">
      <!-- 阅读区域 -->
      <div class="reading-area" :style="readerStyles" ref="readingArea">
        <div v-if="loading" class="loading">
          <div class="loading-spinner"></div>
          <p>加载中...</p>
        </div>
        
        <div v-else class="content-container">
          <!-- 章节标题 -->
          <h1 v-if="currentChapter && showChapterTitle" class="chapter-title">
            {{ currentChapter.title }}
          </h1>
          
          <!-- 文本内容 -->
          <div class="content-text" v-html="formattedContent"></div>
          
          <!-- 翻页提示 -->
          <div class="page-hint" v-if="showPageHint">
            <span v-if="canGoNext">点击右侧或按 → 翻页</span>
            <span v-else-if="hasNextChapter">点击右侧进入下一章</span>
            <span v-else>已到达最后一页</span>
          </div>
        </div>
        
        <!-- 翻页区域 -->
        <div class="page-turn-areas">
          <div class="page-turn-left" @click="previousPage" title="上一页"></div>
          <div class="page-turn-right" @click="nextPage" title="下一页"></div>
        </div>
      </div>

      <!-- 设置面板 -->
      <transition name="slide-left">
        <div v-if="showSettings" class="settings-panel">
          <div class="panel-header">
            <h3>阅读设置</h3>
            <button @click="showSettings = false" class="close-btn">×</button>
          </div>
          
          <div class="settings-content">
            <!-- 字体设置 -->
            <div class="setting-group">
              <label>字体大小</label>
              <div class="range-control">
                <button @click="adjustFontSize(-1)" class="adjust-btn">-</button>
                <input 
                  v-model.number="settings.fontSize" 
                  type="range" 
                  min="12" 
                  max="32" 
                  @input="updateSettings"
                  class="range-input"
                >
                <button @click="adjustFontSize(1)" class="adjust-btn">+</button>
                <span class="value">{{ settings.fontSize }}px</span>
              </div>
            </div>

            <!-- 行间距设置 -->
            <div class="setting-group">
              <label>行间距</label>
              <div class="range-control">
                <button @click="adjustLineHeight(-0.1)" class="adjust-btn">-</button>
                <input 
                  v-model.number="settings.lineHeight" 
                  type="range" 
                  min="1.0" 
                  max="3.0" 
                  step="0.1"
                  @input="updateSettings"
                  class="range-input"
                >
                <button @click="adjustLineHeight(0.1)" class="adjust-btn">+</button>
                <span class="value">{{ settings.lineHeight.toFixed(1) }}</span>
              </div>
            </div>

            <!-- 字体族设置 -->
            <div class="setting-group">
              <label>字体</label>
              <select v-model="settings.fontFamily" @change="updateSettings" class="select-input">
                <option value="system">系统默认</option>
                <option value="serif">宋体</option>
                <option value="sans-serif">黑体</option>
                <option value="monospace">等宽字体</option>
                <option value="'Microsoft YaHei', sans-serif">微软雅黑</option>
                <option value="'PingFang SC', sans-serif">苹方</option>
              </select>
            </div>

            <!-- 主题设置 -->
            <div class="setting-group">
              <label>阅读主题</label>
              <div class="theme-options">
                <button 
                  v-for="theme in themes" 
                  :key="theme.value"
                  @click="setTheme(theme.value)"
                  :class="['theme-btn', { active: settings.theme === theme.value }]"
                  :style="{ backgroundColor: theme.bg, color: theme.color }"
                >
                  {{ theme.name }}
                </button>
              </div>
            </div>

            <!-- 页边距设置 -->
            <div class="setting-group">
              <label>页边距</label>
              <div class="range-control">
                <button @click="adjustMargin(-5)" class="adjust-btn">-</button>
                <input 
                  v-model.number="settings.pageMargin" 
                  type="range" 
                  min="10" 
                  max="80"
                  step="5"
                  @input="updateSettings"
                  class="range-input"
                >
                <button @click="adjustMargin(5)" class="adjust-btn">+</button>
                <span class="value">{{ settings.pageMargin }}px</span>
              </div>
            </div>

            <!-- 页面宽度设置 -->
            <div class="setting-group">
              <label>页面宽度</label>
              <div class="range-control">
                <input 
                  v-model.number="settings.maxWidth" 
                  type="range" 
                  min="600" 
                  max="1200"
                  step="50"
                  @input="updateSettings"
                  class="range-input"
                >
                <span class="value">{{ settings.maxWidth }}px</span>
              </div>
            </div>

            <!-- 其他设置 -->
            <div class="setting-group">
              <label class="checkbox-label">
                <input 
                  type="checkbox" 
                  v-model="settings.showChapterTitle" 
                  @change="updateSettings"
                >
                显示章节标题
              </label>
            </div>

            <div class="setting-group">
              <label class="checkbox-label">
                <input 
                  type="checkbox" 
                  v-model="settings.enablePageAnimation" 
                  @change="updateSettings"
                >
                翻页动画
              </label>
            </div>

            <div class="setting-group">
              <label class="checkbox-label">
                <input 
                  type="checkbox" 
                  v-model="settings.autoSaveProgress" 
                  @change="updateSettings"
                >
                自动保存阅读进度
              </label>
            </div>

            <div class="setting-group">
              <label class="checkbox-label">
                <input 
                  type="checkbox" 
                  v-model="settings.enableKeyboardShortcuts" 
                  @change="updateSettings"
                >
                启用键盘快捷键
              </label>
            </div>

            <div class="setting-group">
              <label class="checkbox-label">
                <input 
                  type="checkbox" 
                  v-model="settings.enableClickTurn" 
                  @change="updateSettings"
                >
                点击翻页
              </label>
            </div>

            <!-- 设置管理 -->
            <div class="setting-group">
              <label>设置管理</label>
              <div class="setting-actions">
                <button @click="resetSettings" class="action-btn reset-btn">
                  重置设置
                </button>
                <button @click="exportSettings" class="action-btn export-btn">
                  导出设置
                </button>
                <label class="action-btn import-btn">
                  导入设置
                  <input 
                    type="file" 
                    accept=".json"
                    @change="importSettings"
                    style="display: none;"
                  >
                </label>
              </div>
            </div>
          </div>
        </div>
      </transition>

      <!-- 搜索面板 -->
      <transition name="slide-left">
        <SearchPanel
          v-if="showSearch"
          :content="content"
          :current-position="readingProgress"
          @close="showSearch = false"
          @go-to-position="goToPosition"
          @highlight-text="highlightSearchText"
        />
      </transition>

      <!-- 书签面板 -->
      <transition name="slide-left">
        <BookmarkPanel
          v-if="showBookmarks"
          :bookmarks="bookmarks"
          :current-position="readingProgress"
          @close="showBookmarks = false"
          @add-bookmark="addBookmark"
          @delete-bookmark="deleteBookmark"
          @go-to-bookmark="goToBookmark"
        />
      </transition>

      <!-- 目录面板 -->
      <transition name="slide-left">
        <div v-if="showToc" class="toc-panel">
          <div class="panel-header">
            <h3>目录</h3>
            <button @click="showToc = false" class="close-btn">×</button>
          </div>
          
          <div class="toc-content">
            <div class="toc-list">
              <div 
                v-for="(chapter, index) in chapters" 
                :key="chapter.id || index"
                @click="goToChapter(index)"
                :class="['toc-item', { active: currentChapterIndex === index }]"
              >
                <span class="chapter-number">{{ index + 1 }}</span>
                <span class="chapter-title">{{ chapter.title || `第${index + 1}章` }}</span>
              </div>
              
              <!-- 如果没有章节数据，显示默认章节 -->
              <div v-if="chapters.length === 0" class="toc-item active">
                <span class="chapter-number">1</span>
                <span class="chapter-title">正文</span>
              </div>
            </div>
          </div>
        </div>
      </transition>
    </div>

    <!-- 阅读器底部 -->
    <div class="reader-footer" v-show="!isFullscreen || showControls">
      <div class="progress-section">
        <div class="progress-info">
          <span class="page-info">{{ currentPage + 1 }} / {{ totalPages }}</span>
          <span class="progress-percent">{{ Math.round(readingProgress * 100) }}%</span>
        </div>
        
        <div class="progress-bar" @click="handleProgressClick">
          <div 
            class="progress-fill" 
            :style="{ width: readingProgress * 100 + '%' }"
          ></div>
        </div>
      </div>
      
      <div class="navigation-controls">
        <button 
          @click="previousChapter" 
          :disabled="!hasPreviousChapter"
          class="nav-btn chapter-btn"
          title="上一章 (Ctrl+←)"
        >
          上一章
        </button>
        
        <button 
          @click="previousPage" 
          :disabled="!canGoPrevious"
          class="nav-btn page-btn"
          title="上一页 (←)"
        >
          上一页
        </button>
        
        <button 
          @click="nextPage" 
          :disabled="!canGoNext && !hasNextChapter"
          class="nav-btn page-btn"
          title="下一页 (→)"
        >
          {{ canGoNext ? '下一页' : (hasNextChapter ? '下一章' : '完') }}
        </button>
        
        <button 
          @click="nextChapter" 
          :disabled="!hasNextChapter"
          class="nav-btn chapter-btn"
          title="下一章 (Ctrl+→)"
        >
          下一章
        </button>
      </div>
    </div>
  </div>

  <!-- 错误状态 -->
  <div v-else class="error-state">
    <div class="error-content">
      <h2>📖</h2>
      <h3>图书未找到</h3>
      <p>请检查图书是否存在或重新选择</p>
      <button @click="$router.push('/library')" class="btn">返回图书库</button>
    </div>
  </div>
</template>

<script>
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'

export default {
  name: 'ReaderView',
  setup() {
    const route = useRoute()
    
    // 基础状态
    const book = ref(null)
    const content = ref('')
    const loading = ref(false)

    
    // UI 状态
    const showSettingsse)
    const showToc = r
    const isFullscre)
    co

    
    // 阅读状态
    const curren= ref(0)
    const totalPages = ref(1)
    const currentChapterIn ref(0)
    const )
    
    // 阅读设置
    const settings = ref({
      font6,
      lineHeight
      fontFamily: 'system',
      theme: 'light',
      pag: 20,
      m: 800,
rue,
      enablePan: true
    })

    // 主题配置
    const themes = ref([
      {,
      },

      { name: '青色', value: 'cyan',}
    ])

算属性
    const c=> {
      return chapters.va: '正文' }
    })

    cons
      return settinlue
    })

    const readingProgres
      if (totalPages.valurn 0
      return currentPage.value value
    })

    const canGoPrevious
      return currentPage.value > 0
    })

    con
     value - 1
   })

    const hasPreviousChapter = computed(() => {
      return currentChaptere > 0
    })

    const hasNextChapter = co(() => {
      return currentChapterIndex.val 1
    })

(() => {
      if (!content.value) return ''
      
      // 简单的文本格式化
     
)
        .replace(/^/, '<p>')
        .replace(/$/, '</p>')
        .replace(/
    })

    c

        light: {
          backgrounffff',
          color: '#33333
     ,

          backgroundColor: 'a',
          color: '#
        },
     a: {
f4f1e8',
          color: '#5c4b37'
        },
        cyan: {
          backgroundColo7fa',
          color '#006064'
        }
      }

      const fontFamily
        system: '-apple-system, Blinserif',
        serif: '"Times New Romaif',
        'sans-s',
       ce'
      }

      return {
        fontSize: see + 'px',
        lineHeig,
        fontFamily: fontFamilyMap[settings.value.fontFamiamily,
      x',
'px',
        margin: '0 auto',
        ...themeStyles[settings.value.theme]
      


    // 方法
    const load {
      const bos.id
      loading.value

      try {
        // 临时数据用于展示
        book.value = 
          id: bookId,
          title: '红楼梦
          author: ',
          read.3
     
    
 / 模拟章节数据
        clue = [
,
          { id},
         }
        ]
        
        // 模拟长文本内容
        content.value = `第一回 甄士隐梦幻闺秀

不惑。

原来女娲氏炼石补天之时，于大荒山

谁知此石自经锻炼之后，灵性已通，因见众石俱得补天，独自己无材不堪入，日夜悲号惭愧。

一日，正当嗟悼之际，俄见一僧一道远远而来，生得骨格不凡，丰神迥异贵。

此万劫不忘也。"

二仙师听毕，齐憨笑道：如不去的好。"

这石凡心已炽，那里听得进这话去切莫后悔。"

石道："自然，自然。"

那僧又道："若说你性灵，却又如此质蠢，并更

石

石头听了，喜不能禁，乃问："不知赐。"

那

后来，又不知过了几世几劫，因有的一段故事。

后面又有一首偈云：

无若许年。
奇传？

诗后便说：后因曹雪芹于悼红轩中披阅

满一把辛酸泪！
者痴，谁解其中味？

出则既明，且看石上是何故事。按那云：

当为望族了。

因这甄士隐禀性恬淡，不以功名

一日，炎夏永昼，士隐于书房闲坐，至手倦抛书，伏几盹睡，不

只听道人问道："你携了这蠢物，意欲何往？"

那僧笑道："你放心，如今现有一段风流公案正该了结，这一干"

处？"

那僧道："此事说来好笑，竟是那些精怪化人，来到世上，爱上

故。"

又有护官符云：

贾不假，白玉为堂金作马。
阿房宫，三百里，住不下金陵一个史。
东海缺少白玉床，龙王来请金陵王。
丰土金如铁。

这四家皆连络有亲，一损皆损，一，俱有照应的。

今且说甄士隐夫妇，因见女儿一日长似一日

急

那士隐夫妇，见女儿一

看看一月，士隐先已哭病，那封氏也因思女构疾。一日，炎

梦至一处，不辨是何地方。忽见那

那僧笑道："你放心，如今现有一段风。"



那僧道："此事说来好笑..."`
        
        calculatePages()
        
 
r)
      } finally 
        loadlse
      }
    }

    const calcul () => {
      // 简化的页数计算
 0

      totalPages.vaPage))
    }

    // 设置相关方法
    const toggleSettings = () => {
      showSettings.valu
 e
 }

    const toggleToc = => {
 lue
ue = false
    }

    const toggleFullscre=> {
      isFullscreelue
      
 e) {
n?.()
      } else {
        documen?.()
      }
    }

    const setTheme = (t{
      settingse
      updateSettin
    }

 > {
))
      setti
      updateSettings
    }


      const
      settings.valu/ 10
      updateSettings()
    }

 {

      settings.vaewMargin
      updateSettings()
 

    const update
      localStorage.sue))
      calculatePages()
    }

    const loadSettings{
      try {
 s')
 {
          settind) }
        }
      } catch (error) {
        console.er
 }
    }

    // 翻页相关方法
    const prev
      if (canGoPrevious.valu {
        currentPage.v-
        saveReading
 {
()
      }
    }

    const nextPage = () => {
 alue) {
ue++
        saveR)
      } else if ) {
        nextCh()
 
    }

    const previousChap=> {
      if (hasPr{
        currentlue--
        currentPage.val= 0
        // 这里应该加载新章节内容
        saveReadin
      }
 

    const nextCh=> {
      if (hasNextChapter.val
 e++
 0
        // 这里应容
        saveRead
      }
    }

    const goToChex) => {
      if (in
 

        showToc.vfalse
        // 这里应该加载
 ress()

    }

    const handlt) => {
      const rec
      const clickX = eventrect.left
      const progress t.width
      const target)
      
      currentPage.value = Math.max(0, Matage))
 
    }

    const saveReadingProgres=> {
 return
        t>rip
</sc }
}
    }
 tClickConten  handle
    k,ogressClichandlePr   ,
   apteroToCher,
      gtChapt   nex,
   pterviousCha
      prextPage,   nege,
   Pa    previousettings,
    updateS
    in,Margdjustht,
      aineHeigstLadjuize,
      tSadjustFone,
           setThemn,
 reelsc  toggleFul
    toggleToc,  ings,
      toggleSett// 方法
       
      
   rStyles,readet,
      edContenormatt   fter,
   xtChapsNe     hasChapter,
 sPreviou    hat,
      canGoNexs,
  GoPreviou    cans,
  esingProgr     readterTitle,
 wChap,
      shoapterntChcurre       // 计算属性
   
         themes,
gs,
        settinpters,
        chadex,
 pterInentCha curr
     es,lPagota      t
currentPage,     ageHint,
 howP    s  s,
ontrol      showCen,
isFullscreoc,
           showT
 Settings,showea,
      Areading      rg,
  loadin
    nt,
      conte      book,/ 状态
    /
     return { })

 mer)
   eControlsTiimeout(hidrT      cleaove)
andleMouseMve', husemor('moentListene.removeEv   documentydown)
   dleKeydown', hanListener('keveEventocument.remo  d
    => {() unted(
    onUnmo
    })
   }, 1000))
    3000  },e
      lue = falsageHint.va     showP
     ut(() => {meo   setTi    
 ue = trueHint.val  showPage> {
      t(() =imeousetT提示
          // 显示翻页    
  e)
  ovndleMouseMusemove', har('moventListenement.addE      docu)
own handleKeyd'keydown',tListener(ent.addEven     docum
      
 gProgress()in   loadReaddBook()
      await loa)
   ngs(Setti load{
      => sync ()(aounted命周期
    onM 生//     }

 
  mer()deControlsTitHi   rese
   => {ve = () useMot handleMo

    cons    }00)
 }, 30se
     alue = falols.v showContr
       (() => {imeout setTer =ntrolsTimeCo)
      hidimerontrolsTideCearTimeout(h
      cle = true.valurols   showCont     
   e) return
 en.valuisFullscreif (! {
      ) =>= (imer trolsTsetHideCon  const re  er = null
olsTimdeContrhi）
    let 控制栏（全屏模式下 // 自动隐藏   }

   
      }
     breaklt()
      preventDefauent.     evage()
       nextP      ':
    '     case  break
          
 lt()fau.preventDe   event     screen()
   toggleFull        11':
      case 'F   k
       brea    }
   
      ()efaultnt.preventD eve
           gleToc()         toglKey) {
   ctr (!event.    if':
       'T  case  
    t': '     casek
         brea }
    
         ult()eventDefat.pr    even)
        ettings(    toggleS     
   ctrlKey) { (!event.      if    :
    case 'S' 's':
     case
         break             }
 )
    ullscreen(    toggleF     {
    ue)een.valisFullscr   if (se
        = falc.valueowTo   sh     
  ue = false.valgsinhowSett         scape':
 Es    case 'ak
        bre
      ault()t.preventDef   even            }
  
   e()   nextPag     lse {
             } e()
 hapterxtC   ne       {
  y) lKe (event.ctr   if    
   ArrowRight':     case 'eak
             br)
ault(entDeft.prev    even   }
      ()
       ge  previousPa          {
 else         } )
 ter(reviousChap     p    y) {
   Kent.ctrl    if (eveft':
      wLee 'Arro        cas{
nt.key)  (eve   switch

       }return
   {
        'TEXTAREA')gName === t.ta.targe| event== 'INPUT' |tagName =get.arnt.teve if (处理快捷键
      // 如果正在输入，不
     ent) => {wn = (evandleKeydo  const h
  
    }
  }e()
    nextPag
         / 2) {+ centerX> centerX clickX  else if ()
      }reviousPage(        p
X / 2) {ckX < center   if (cli    
   2
   / t.widthrX = rect centenscot
      X - rect.lefient event.clX =onst click      c    
urn
  !rect) ret      if (()
ntRectBoundingClie.gete?luingArea.va rect = readconst{
      =>  (event) tClick =dleContennst han    co// 事件处理
   

  }
    }ror)
     进度失败:', err('加载阅读rro.e console{
       tch (error) ca
      }  }   
    e || 0Data.pag= progressue valentPage.curr        er || 0
  aptessData.chalue = progr.vhapterIndexntC     curreaved)
     parse(sN.JSOata = essD progronst  c  ) {
      (saved if      }`)
  .id{book.valuerogress_$(`reading_prage.getItemocalStoved = l    const sary {
      
      t   return
  alue)k.v if (!boo   ) => {
  ogress = (gPrinloadReadst 
    con}
}
          )
rror阅读进度失败:', eerror('保存nsole.        co {
r)atch (erro      } ca))
ogressDattringify(prJSON.sid}`, alue.ess_${book.vogrding_prItem(`rearage.setalSto     loc    }
      .now()
 amp: Date   timest       age.value,
: currentP  page
        ndex.value,ntChapterI: curreapter        chs,
  rogres        p
  id,alue.: book.v      bookId  = {
  ressData   const prog存储
      到本地   // 临时保存   
     // })
           
  uealrrentPage.vage: cu    //   p  ,
  ex.valuetChapterIndenpter: curr //   cha       
ss,grero p      //  e.id, 
  lu.va: book/   bookId
        /{ s', ing_progres_readsavet invoke('ai/ aw        /PI保存进度
后端A/ 这里应该调用     /ue
   ogress.val= readingPrress ogpr   const {
      try 
   <scrip
t>
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import SearchPanel from '../components/SearchPanel.vue'
import BookmarkPanel from '../components/BookmarkPanel.vue'
import { 
  settingsManager, 
  progressManager, 
  READER_THEMES, 
  getThemeStyles, 
  getFontFamilyStyle,
  validateSettingsValue 
} from '../utils/settingsManager.js'

export default {
  components: {
    SearchPanel,
    BookmarkPanel
  },
  name: 'ReaderView',
  setup() {
    const route = useRoute()
    
    // 基础状态
    const book = ref(null)
    const content = ref('')
    const loading = ref(false)
    const readingArea = ref(null)
    
    // UI 状态
    const showSettings = ref(false)
    const showToc = ref(false)
    const showSearch = ref(false)
    const showBookmarks = ref(false)
    const isFullscreen = ref(false)
    const showControls = ref(true)
    const showPageHint = ref(false)
    
    // 阅读状态
    const currentPage = ref(0)
    const totalPages = ref(1)
    const currentChapterIndex = ref(0)
    const chapters = ref([])
    const bookmarks = ref([])
    const searchHighlight = ref('')
    
    // 阅读设置
    const settings = ref(settingsManager.load())

    // 主题配置
    const themes = ref(READER_THEMES)

    // 计算属性
    const currentChapter = computed(() => {
      return chapters.value[currentChapterIndex.value] || { title: '正文' }
    })

    const showChapterTitle = computed(() => {
      return settings.value.showChapterTitle && currentChapter.value
    })

    const readingProgress = computed(() => {
      if (totalPages.value === 0) return 0
      return currentPage.value / totalPages.value
    })

    const canGoPrevious = computed(() => {
      return currentPage.value > 0
    })

    const canGoNext = computed(() => {
      return currentPage.value < totalPages.value - 1
    })

    const hasPreviousChapter = computed(() => {
      return currentChapterIndex.value > 0
    })

    const hasNextChapter = computed(() => {
      return currentChapterIndex.value < chapters.value.length - 1
    })

    const formattedContent = computed(() => {
      if (!content.value) return ''
      
      // 简单的文本格式化
      return content.value
        .replace(/\n\s*\n/g, '</p><p>')
        .replace(/^/, '<p>')
        .replace(/$/, '</p>')
        .replace(/<p><\/p>/g, '')
    })

    const readerStyles = computed(() => {
      return {
        fontSize: settings.value.fontSize + 'px',
        lineHeight: settings.value.lineHeight,
        fontFamily: getFontFamilyStyle(settings.value.fontFamily),
        padding: settings.value.pageMargin + 'px',
        maxWidth: settings.value.maxWidth + 'px',
        margin: '0 auto',
        ...getThemeStyles(settings.value.theme)
      }
    })

    // 方法
    const loadBook = async () => {
      const bookId = route.params.id
      loading.value = true

      try {
        // 临时数据用于展示
        book.value = {
          id: bookId,
          title: '红楼梦',
          author: '曹雪芹',
          reading_progress: 0.3
        }
        
        // 模拟章节数据
        chapters.value = [
          { id: '1', title: '第一回 甄士隐梦幻识通灵 贾雨村风尘怀闺秀' },
          { id: '2', title: '第二回 贾夫人仙逝扬州城 冷子兴演说荣国府' },
          { id: '3', title: '第三回 托内兄如海荐西宾 接外孙贾母惜孤女' }
        ]
        
        // 模拟长文本内容
        content.value = `第一回 甄士隐梦幻识通灵 贾雨村风尘怀闺秀

列位看官：你道此书从何而来？说起根由虽近荒唐，细按则深有趣味。待在下将此来历注明，方使阅者了然不惑。

原来女娲氏炼石补天之时，于大荒山无稽崖练成高经十二丈、方经二十四丈顽石三万六千五百零一块。娲皇氏只用了三万六千五百块，只单单剩了一块未用，便弃在此山青埂峰下。

谁知此石自经锻炼之后，灵性已通，因见众石俱得补天，独自己无材不堪入选，遂自怨自叹，日夜悲号惭愧。

一日，正当嗟悼之际，俄见一僧一道远远而来，生得骨格不凡，丰神迥异，说说笑笑来至峰下，坐于石边高谈快论。先是说些云山雾海神仙玄幻之事，后便说到红尘中荣华富贵。

此石听了，不觉打动凡心，也想要到人间去享一享这荣华富贵，但自恨粗蠢，不得已，便口吐人言，向那僧道说道："大师，弟子蠢物，不能见礼了。适闻二位谈那人世间荣耀繁华，心切慕之。弟子质虽粗蠢，性却稍通，况见二师仙形道体，必有补天济世之材，如蒙发一点慈心，携带弟子得入红尘，在那富贵场中、温柔乡里受享几年，自当永佩洪恩，万劫不忘也。"

二仙师听毕，齐憨笑道："善哉，善哉！那红尘中有却有些乐事，但不能永远依恃，况又有'美中不足，好事多磨'八个字紧相连属，瞬息间则又乐极悲生，人非物换，究竟是到头一梦，万境归空，倒不如不去的好。"

这石凡心已炽，那里听得进这话去，乃复苦求再四。二仙知不可强制，乃叹道："此亦静极思动，无中生有之数也。既如此，我们便携你去受享受享，只是到不得意时，切莫后悔。"

石道："自然，自然。"

那僧又道："若说你性灵，却又如此质蠢，并更无奇贵之处，如此也只好踮脚而已。也罢，我如今大施佛法助你助你，待劫终之日，复还本质，以了此案。你道好否？"

石头听了，感谢不尽。那僧便念咒书符，大展幻术，将一块大石登时变成一块鲜明莹洁的美玉，且又缩成扇坠大小的可佩可拿。那僧托于掌上，笑道："形体倒也是个宝物了！还只没有实在的好处，须得再镌上数字，使人一见便知是奇物方妙。然后携你到那昌明隆盛之邦，诗礼簪缨之族，花柳繁华地，温柔富贵乡去安身乐业。"

石头听了，喜不能禁，乃问："不知赐了弟子那几件奇处，又不知携了弟子到何地方？望乞明示，使弟子不惑。"

那僧笑道："你且莫问，日后自然明白的。"说着，便袖了这石，同那道人飘然而去，竟不知投奔何方何舍。

后来，又不知过了几世几劫，因有个空空道人访道求仙，忽从这大荒山无稽崖青埂峰下经过，忽见一大块石上字迹分明，编述历历。空空道人乃从头一看，原来就是无材补天，幻形入世，蒙茫茫大士、渺渺真人携入红尘，历尽离合悲欢炎凉世态的一段故事。

后面又有一首偈云：

无材可去补苍天，枉入红尘若许年。
此系身前身后事，倩谁记去作奇传？

诗后便说：后因曹雪芹于悼红轩中披阅十载，增删五次，纂成目录，分出章回，则题曰《金陵十二钗》。并题一绝云：

满纸荒唐言，一把辛酸泪！
都云作者痴，谁解其中味？

出则既明，且看石上是何故事。按那石上书云：

当日地陷东南，这东南一隅有处曰姑苏，有城曰阊门者，最是红尘中一二等富贵风流之地。这阊门外有个十里街，街内有个仁清巷，巷内有个古庙，因地方窄狭，人皆呼作葫芦庙。庙旁住着一家乡宦，姓甄，名费，字士隐。嫡妻封氏，情性贤淑，深明礼义。家中虽不甚富贵，然本地便也推他为望族了。

因这甄士隐禀性恬淡，不以功名为念，每日只以观花修竹、酌酒吟诗为乐，倒是神仙一流人品。只是一件不足：如今年已半百，膝下无儿，只有一女，乳名英莲，年方三岁，生得粉妆玉琢，乖觉可喜。

一日，炎夏永昼，士隐于书房闲坐，至手倦抛书，伏几盹睡，不觉朦胧睡去。梦至一处，不辨是何地方，忽见那厢来了一僧一道，且行且谈。

只听道人问道："你携了这蠢物，意欲何往？"

那僧笑道："你放心，如今现有一段风流公案正该了结，这一干风流冤家，尚未投胎入世。趁此机会，就将此蠢物夹带于中，使他去经历经历。"

那道人道："原来近日风流冤孽又将造劫历世去不成？但不知落于何方何处？"

那僧道："此事说来好笑，竟是那些精怪化人，来到世上，爱上几个美人，恋恋不舍，满心中意淫意恶，遂凝结成胎，化作此山中这一块顽石，上面撰满了他半世亲见亲闻的这些女子，便借通灵之说，编述一集，以告天下人。虽我未学，下笔无文，又何妨用假语村言，敷演出一段故事来，亦可使闺阁昭传，复可悦世之目，破人愁闷，不亦宜乎？"

故曰："贾不假，白玉为堂金作马。阿房宫，三百里，住不下金陵一个史。东海缺少白玉床，龙王来请金陵王。丰年好大雪，珍珠如土金如铁。"

又有护官符云：

贾不假，白玉为堂金作马。
阿房宫，三百里，住不下金陵一个史。
东海缺少白玉床，龙王来请金陵王。
丰年好大雪，珍珠如土金如铁。

这四家皆连络有亲，一损皆损，一荣皆荣，扶持遮饰，俱有照应的。

今且说甄士隐夫妇，因见女儿一日长似一日，生得袅娜纤巧，不胜怜爱，便是稍有些不遂心的事，见了女儿的乖巧可爱，便连烦恼也忘了。这日正是元宵佳节，士隐命家人霍启抱了英莲去看社火花灯，半夜中霍启因要小解，便将英莲放在一家门槛上坐着，待他小解完了来抱时，哪有英莲的踪影？

急得霍启直寻了半夜，至天明不见，那霍启也就不敢回来见主人，便逃往他乡去了。

那士隐夫妇，见女儿一夜不归，便知有些不妥，再使几个人去寻找，回来皆云连音信皆无。夫妻二人，半世只生此女，一旦失落，岂不思想，因此昼夜啼哭，几乎不曾寻死。

看看一月，士隐先已哭病，那封氏也因思女构疾。一日，炎夏永昼，士隐坐于书房中，至手倦抛书，伏几少憩，不觉朦胧睡去。

梦至一处，不辨是何地方。忽见那厢来了一僧一道，且行且谈。只听道人问道："你携了这蠢物，意欲何往？"

那僧笑道："你放心，如今现有一段风流公案正该了结，这一干风流冤家，尚未投胎入世。趁此机会，就将此蠢物夹带于中，使他去经历经历。"

那道人道："原来近日风流冤孽又将造劫历世去不成？但不知落于何方何处？"

那僧道："此事说来好笑..."`
        
        calculatePages()
        
      } catch (error) {
        console.error('加载图书失败:', error)
      } finally {
        loading.value = false
      }
    }

    const calculatePages = () => {
      // 简化的页数计算
      const wordsPerPage = 800
      const wordCount = content.value.length
      totalPages.value = Math.max(1, Math.ceil(wordCount / wordsPerPage))
    }

    // 设置相关方法
    const toggleSettings = () => {
      showSettings.value = !showSettings.value
      showToc.value = false
    }

    const toggleToc = () => {
      showToc.value = !showToc.value
      showSettings.value = false
      showSearch.value = false
      showBookmarks.value = false
    }

    const toggleSearch = () => {
      showSearch.value = !showSearch.value
      showSettings.value = false
      showToc.value = false
      showBookmarks.value = false
    }

    const toggleBookmarks = () => {
      showBookmarks.value = !showBookmarks.value
      showSettings.value = false
      showToc.value = false
      showSearch.value = false
    }

    const toggleFullscreen = () => {
      isFullscreen.value = !isFullscreen.value
      
      if (isFullscreen.value) {
        document.documentElement.requestFullscreen?.()
      } else {
        document.exitFullscreen?.()
      }
    }

    const setTheme = (theme) => {
      settings.value.theme = theme
      updateSettings()
    }

    const adjustFontSize = (delta) => {
      const newSize = validateSettingsValue('fontSize', settings.value.fontSize + delta)
      settings.value.fontSize = newSize
      updateSettings()
    }

    const adjustLineHeight = (delta) => {
      const newHeight = validateSettingsValue('lineHeight', settings.value.lineHeight + delta)
      settings.value.lineHeight = Math.round(newHeight * 10) / 10
      updateSettings()
    }

    const adjustMargin = (delta) => {
      const newMargin = validateSettingsValue('pageMargin', settings.value.pageMargin + delta)
      settings.value.pageMargin = newMargin
      updateSettings()
    }

    const updateSettings = () => {
      settingsManager.update(settings.value)
      calculatePages()
    }

    const loadSettings = () => {
      settings.value = settingsManager.load()
    }

    const resetSettings = () => {
      settingsManager.reset()
      settings.value = settingsManager.get()
      calculatePages()
    }

    const exportSettings = () => {
      const data = settingsManager.export()
      const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `reader-settings-${new Date().toISOString().split('T')[0]}.json`
      document.body.appendChild(a)
      a.click()
      document.body.removeChild(a)
      URL.revokeObjectURL(url)
    }

    const importSettings = (event) => {
      const file = event.target.files[0]
      if (!file) return
      
      const reader = new FileReader()
      reader.onload = (e) => {
        try {
          const data = JSON.parse(e.target.result)
          if (settingsManager.import(data)) {
            settings.value = settingsManager.get()
            calculatePages()
            console.log('设置导入成功')
          } else {
            console.error('设置文件格式不正确')
          }
        } catch (error) {
          console.error('导入设置失败:', error)
        }
      }
      reader.readAsText(file)
    }

    // 翻页相关方法
    const previousPage = () => {
      if (canGoPrevious.value) {
        currentPage.value--
        saveReadingProgress()
      } else if (hasPreviousChapter.value) {
        previousChapter()
      }
    }

    const nextPage = () => {
      if (canGoNext.value) {
        currentPage.value++
        saveReadingProgress()
      } else if (hasNextChapter.value) {
        nextChapter()
      }
    }

    const previousChapter = () => {
      if (hasPreviousChapter.value) {
        currentChapterIndex.value--
        currentPage.value = 0
        // 这里应该加载新章节内容
        saveReadingProgress()
      }
    }

    const nextChapter = () => {
      if (hasNextChapter.value) {
        currentChapterIndex.value++
        currentPage.value = 0
        // 这里应该加载新章节内容
        saveReadingProgress()
      }
    }

    const goToChapter = (index) => {
      if (index >= 0 && index < chapters.value.length) {
        currentChapterIndex.value = index
        currentPage.value = 0
        showToc.value = false
        // 这里应该加载新章节内容
        saveReadingProgress()
      }
    }

    const handleProgressClick = (event) => {
      const rect = event.currentTarget.getBoundingClientRect()
      const clickX = event.clientX - rect.left
      const progress = clickX / rect.width
      const targetPage = Math.floor(progress * totalPages.value)
      
      currentPage.value = Math.max(0, Math.min(totalPages.value - 1, targetPage))
      saveReadingProgress()
    }

    const goToPosition = (position) => {
      const targetPage = Math.floor(position * totalPages.value)
      currentPage.value = Math.max(0, Math.min(totalPages.value - 1, targetPage))
      saveReadingProgress()
    }

    // 书签功能
    const loadBookmarks = async () => {
      if (!book.value) return
      
      try {
        // 这里应该调用后端API加载书签
        // const bookmarksData = await invoke('get_bookmarks', { bookId: book.value.id })
        
        // 临时从本地存储加载
        const saved = localStorage.getItem(`bookmarks_${book.value.id}`)
        if (saved) {
          bookmarks.value = JSON.parse(saved)
        }
      } catch (error) {
        console.error('加载书签失败:', error)
      }
    }

    const addBookmark = async (bookmarkData) => {
      if (!book.value) return
      
      try {
        const bookmark = {
          id: Date.now().toString(),
          bookId: book.value.id,
          position: bookmarkData.position,
          note: bookmarkData.note || '',
          content_preview: getContentPreview(bookmarkData.position),
          created_at: Date.now()
        }
        
        // 这里应该调用后端API保存书签
        // await invoke('add_bookmark', bookmark)
        
        // 临时保存到本地存储
        bookmarks.value.push(bookmark)
        localStorage.setItem(`bookmarks_${book.value.id}`, JSON.stringify(bookmarks.value))
        
        console.log('书签添加成功')
      } catch (error) {
        console.error('添加书签失败:', error)
      }
    }

    const deleteBookmark = async (bookmarkId) => {
      try {
        // 这里应该调用后端API删除书签
        // await invoke('delete_bookmark', { bookmarkId })
        
        // 临时从本地存储删除
        bookmarks.value = bookmarks.value.filter(b => b.id !== bookmarkId)
        localStorage.setItem(`bookmarks_${book.value.id}`, JSON.stringify(bookmarks.value))
        
        console.log('书签删除成功')
      } catch (error) {
        console.error('删除书签失败:', error)
      }
    }

    const goToBookmark = (bookmark) => {
      goToPosition(bookmark.position)
      showBookmarks.value = false
    }

    const getContentPreview = (position) => {
      if (!content.value) return ''
      
      const index = Math.floor(position * content.value.length)
      const start = Math.max(0, index - 50)
      const end = Math.min(content.value.length, index + 100)
      
      return content.value.substring(start, end).trim()
    }

    // 搜索功能
    const highlightSearchText = (searchText, options) => {
      searchHighlight.value = searchText
      // 这里可以添加更复杂的高亮逻辑
    }

    const saveReadingProgress = async () => {
      if (!book.value) return
      
      try {
        const progressData = {
          bookId: book.value.id,
          progress: readingProgress.value,
          chapter: currentChapterIndex.value,
          page: currentPage.value,
          position: readingProgress.value,
          totalPages: totalPages.value
        }
        
        // 使用进度管理器保存
        progressManager.save(book.value.id, progressData)
        
        // 如果有后端API，也可以同时保存到服务器
        // await invoke('save_reading_progress', progressData)
      } catch (error) {
        console.error('保存阅读进度失败:', error)
      }
    }

    const loadReadingProgress = () => {
      if (!book.value) return
      
      try {
        const progressData = progressManager.load(book.value.id)
        if (progressData) {
          currentChapterIndex.value = progressData.chapter || 0
          currentPage.value = progressData.page || 0
        }
      } catch (error) {
        console.error('加载阅读进度失败:', error)
      }
    }

    const startAutoSave = () => {
      if (!book.value || !settings.value.autoSaveProgress) return
      
      const getProgressData = () => ({
        bookId: book.value.id,
        progress: readingProgress.value,
        chapter: currentChapterIndex.value,
        page: currentPage.value,
        position: readingProgress.value,
        totalPages: totalPages.value
      })
      
      progressManager.startAutoSave(
        book.value.id, 
        getProgressData, 
        settings.value.saveInterval
      )
    }

    const stopAutoSave = () => {
      progressManager.stopAutoSave()
    }

    // 事件处理
    const handleContentClick = (event) => {
      const rect = readingArea.value?.getBoundingClientRect()
      if (!rect) return
      
      const clickX = event.clientX - rect.left
      const centerX = rect.width / 2
      
      if (clickX < centerX / 2) {
        previousPage()
      } else if (clickX > centerX + centerX / 2) {
        nextPage()
      }
    }

    const handleKeydown = (event) => {
      // 如果正在输入，不处理快捷键
      if (event.target.tagName === 'INPUT' || event.target.tagName === 'TEXTAREA') {
        return
      }

      switch (event.key) {
        case 'ArrowLeft':
          if (event.ctrlKey) {
            previousChapter()
          } else {
            previousPage()
          }
          event.preventDefault()
          break
        case 'ArrowRight':
          if (event.ctrlKey) {
            nextChapter()
          } else {
            nextPage()
          }
          event.preventDefault()
          break
        case 'Escape':
          showSettings.value = false
          showToc.value = false
          showSearch.value = false
          showBookmarks.value = false
          if (isFullscreen.value) {
            toggleFullscreen()
          }
          break
        case 'f':
        case 'F':
          if (event.ctrlKey) {
            toggleSearch()
            event.preventDefault()
          }
          break
        case 'b':
        case 'B':
          if (!event.ctrlKey) {
            toggleBookmarks()
            event.preventDefault()
          }
          break
        case 's':
        case 'S':
          if (!event.ctrlKey) {
            toggleSettings()
            event.preventDefault()
          }
          break
        case 't':
        case 'T':
          if (!event.ctrlKey) {
            toggleToc()
            event.preventDefault()
          }
          break
        case 'F11':
          toggleFullscreen()
          event.preventDefault()
          break
        case ' ':
          nextPage()
          event.preventDefault()
          break
      }
    }

    // 自动隐藏控制栏（全屏模式下）
    let hideControlsTimer = null
    const resetHideControlsTimer = () => {
      if (!isFullscreen.value) return
      
      showControls.value = true
      clearTimeout(hideControlsTimer)
      hideControlsTimer = setTimeout(() => {
        showControls.value = false
      }, 3000)
    }

    const handleMouseMove = () => {
      resetHideControlsTimer()
    }

    // 生命周期
    onMounted(async () => {
      loadSettings()
      await loadBook()
      loadReadingProgress()
      await loadBookmarks()
      
      // 启动自动保存
      startAutoSave()
      
      document.addEventListener('keydown', handleKeydown)
      document.addEventListener('mousemove', handleMouseMove)
      
      // 显示翻页提示
      setTimeout(() => {
        showPageHint.value = true
        setTimeout(() => {
          showPageHint.value = false
        }, 3000)
      }, 1000)
    })

    onUnmounted(() => {
      // 停止自动保存
      stopAutoSave()
      
      // 最后保存一次进度
      saveReadingProgress()
      
      document.removeEventListener('keydown', handleKeydown)
      document.removeEventListener('mousemove', handleMouseMove)
      clearTimeout(hideControlsTimer)
    })

    return {
      // 状态
      book,
      content,
      loading,
      readingArea,
      showSettings,
      showToc,
      showSearch,
      showBookmarks,
      isFullscreen,
      showControls,
      showPageHint,
      currentPage,
      totalPages,
      currentChapterIndex,
      chapters,
      bookmarks,
      searchHighlight,
      settings,
      themes,
      
      // 计算属性
      currentChapter,
      showChapterTitle,
      readingProgress,
      canGoPrevious,
      canGoNext,
      hasPreviousChapter,
      hasNextChapter,
      formattedContent,
      readerStyles,
      
      // 方法
      toggleSettings,
      toggleToc,
      toggleSearch,
      toggleBookmarks,
      toggleFullscreen,
      setTheme,
      adjustFontSize,
      adjustLineHeight,
      adjustMargin,
      updateSettings,
      previousPage,
      nextPage,
      previousChapter,
      nextChapter,
      goToChapter,
      goToPosition,
      handleProgressClick,
      handleContentClick,
      addBookmark,
      deleteBookmark,
      goToBookmark,
      highlightSearchText,
      resetSettings,
      exportSettings,
      importSettings
    }
  }
}
</script>

<style scoped>
/* 基础样式 */
.reader {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
  position: relative;
  overflow: hidden;
}

.reader.fullscreen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 9999;
}

/* 头部样式 */
.reader-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 2rem;
  background: var(--bg-primary);
  border-bottom: 1px solid var(--border-color);
  transition: all 0.3s ease;
  z-index: 100;
}

.back-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background: none;
  border: none;
  font-size: 1rem;
  cursor: pointer;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  transition: all 0.3s ease;
  color: var(--text-primary);
}

.back-btn:hover {
  background-color: var(--sidebar-hover);
}

.back-btn .icon {
  font-size: 1.2rem;
}

.book-info {
  flex: 1;
  text-align: center;
  margin: 0 2rem;
}

.book-info h2 {
  margin: 0 0 0.25rem 0;
  color: var(--text-primary);
  font-size: 1.2rem;
  font-weight: 600;
}

.author {
  color: var(--text-secondary);
  font-size: 0.9rem;
  display: block;
  margin-bottom: 0.25rem;
}

.chapter-info {
  color: var(--text-muted);
  font-size: 0.8rem;
  display: block;
}

.reader-controls {
  display: flex;
  gap: 0.5rem;
}

.control-btn {
  display: flex;
  align-items: center;
  background: none;
  border: 1px solid var(--border-color);
  padding: 0.5rem;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.3s ease;
  color: var(--text-secondary);
  min-width: 40px;
  justify-content: center;
}

.control-btn:hover {
  background-color: var(--sidebar-hover);
  border-color: var(--accent-color);
  color: var(--accent-color);
}

.control-btn .icon {
  font-size: 1.1rem;
}

/* 主内容区域 */
.reader-content {
  flex: 1;
  display: flex;
  position: relative;
  overflow: hidden;
}

.reading-area {
  flex: 1;
  overflow-y: auto;
  transition: all 0.3s ease;
  position: relative;
  cursor: pointer;
}

.content-container {
  min-height: 100%;
  position: relative;
}

.chapter-title {
  text-align: center;
  margin: 2rem 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--text-primary);
  border-bottom: 2px solid var(--border-color);
  padding-bottom: 1rem;
}

.content-text {
  text-align: justify;
  word-wrap: break-word;
  hyphens: auto;
}

.content-text p {
  margin: 1em 0;
  text-indent: 2em;
}

/* 加载状态 */
.loading {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
  color: var(--text-secondary);
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--border-color);
  border-top: 3px solid var(--accent-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* 翻页区域 */
.page-turn-areas {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  z-index: 1;
}

.page-turn-left,
.page-turn-right {
  position: absolute;
  top: 0;
  bottom: 0;
  width: 25%;
  pointer-events: all;
  cursor: pointer;
}

.page-turn-left {
  left: 0;
}

.page-turn-right {
  right: 0;
}

.page-turn-left:hover,
.page-turn-right:hover {
  background: rgba(0, 0, 0, 0.05);
}

/* 翻页提示 */
.page-hint {
  position: fixed;
  bottom: 100px;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(0, 0, 0, 0.8);
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 20px;
  font-size: 0.9rem;
  z-index: 1000;
  animation: fadeInOut 3s ease-in-out;
}

@keyframes fadeInOut {
  0%, 100% { opacity: 0; }
  20%, 80% { opacity: 1; }
}

/* 侧边面板 */
.settings-panel,
.toc-panel {
  width: 320px;
  background: var(--bg-primary);
  border-left: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  z-index: 50;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.panel-header h3 {
  margin: 0;
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--text-secondary);
  padding: 0.25rem;
  border-radius: 4px;
  transition: all 0.3s ease;
}

.close-btn:hover {
  background-color: var(--sidebar-hover);
  color: var(--text-primary);
}

.settings-content,
.toc-content {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

/* 设置组件 */
.setting-group {
  margin-bottom: 2rem;
}

.setting-group label {
  display: block;
  margin-bottom: 0.75rem;
  color: var(--text-primary);
  font-weight: 500;
  font-size: 0.9rem;
}

.range-control {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.adjust-btn {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  width: 32px;
  height: 32px;
  border-radius: 6px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  color: var(--text-primary);
  transition: all 0.3s ease;
}

.adjust-btn:hover {
  background-color: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.range-input {
  flex: 1;
  height: 6px;
  background: var(--border-color);
  border-radius: 3px;
  outline: none;
  -webkit-appearance: none;
}

.range-input::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 18px;
  height: 18px;
  background: var(--accent-color);
  border-radius: 50%;
  cursor: pointer;
}

.range-input::-moz-range-thumb {
  width: 18px;
  height: 18px;
  background: var(--accent-color);
  border-radius: 50%;
  cursor: pointer;
  border: none;
}

.value {
  min-width: 60px;
  text-align: right;
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.select-input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 0.9rem;
}

.select-input:focus {
  outline: none;
  border-color: var(--accent-color);
}

/* 主题选择 */
.theme-options {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.5rem;
}

.theme-btn {
  padding: 0.75rem;
  border: 2px solid transparent;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  font-weight: 500;
  transition: all 0.3s ease;
  text-align: center;
}

.theme-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.theme-btn.active {
  border-color: var(--accent-color);
  box-shadow: 0 0 0 1px var(--accent-color);
}

/* 复选框 */
.checkbox-label {
  display: flex !important;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  margin-bottom: 0 !important;
}

.checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  accent-color: var(--accent-color);
}

/* 设置操作按钮 */
.setting-actions {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.action-btn {
  padding: 0.5rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
  text-align: center;
  transition: all 0.3s ease;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.action-btn:hover {
  background-color: var(--sidebar-hover);
  border-color: var(--accent-color);
}

.reset-btn:hover {
  background-color: var(--error-color);
  color: white;
  border-color: var(--error-color);
}

.export-btn:hover {
  background-color: var(--success-color);
  color: white;
  border-color: var(--success-color);
}

.import-btn {
  position: relative;
  overflow: hidden;
}

.import-btn:hover {
  background-color: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

/* 目录 */
.toc-list {
  max-height: none;
}

.toc-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.3s ease;
  margin-bottom: 0.25rem;
}

.toc-item:hover {
  background-color: var(--sidebar-hover);
}

.toc-item.active {
  background-color: var(--accent-color);
  color: white;
}

.chapter-number {
  min-width: 24px;
  height: 24px;
  background: var(--border-color);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.8rem;
  font-weight: 600;
}

.toc-item.active .chapter-number {
  background: rgba(255, 255, 255, 0.2);
  color: white;
}

.chapter-title {
  flex: 1;
  font-size: 0.9rem;
  line-height: 1.4;
}

/* 底部控制栏 */
.reader-footer {
  background: var(--bg-primary);
  border-top: 1px solid var(--border-color);
  padding: 1rem 2rem;
  display: flex;
  align-items: center;
  gap: 2rem;
  z-index: 100;
}

.progress-section {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 1rem;
}

.progress-info {
  display: flex;
  align-items: center;
  gap: 1rem;
  font-size: 0.9rem;
  color: var(--text-secondary);
  min-width: 120px;
}

.progress-bar {
  flex: 1;
  height: 6px;
  background-color: var(--border-color);
  border-radius: 3px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.3s ease;
}

.progress-bar:hover {
  height: 8px;
}

.progress-fill {
  height: 100%;
  background-color: var(--accent-color);
  transition: width 0.3s ease;
}

.navigation-controls {
  display: flex;
  gap: 0.5rem;
}

.nav-btn {
  background: var(--accent-color);
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  font-weight: 500;
  transition: all 0.3s ease;
  min-width: 70px;
}

.nav-btn:hover:not(:disabled) {
  background-color: var(--accent-hover);
  transform: translateY(-1px);
}

.nav-btn:disabled {
  background-color: var(--border-color);
  color: var(--text-muted);
  cursor: not-allowed;
  transform: none;
}

.chapter-btn {
  background: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.chapter-btn:hover:not(:disabled) {
  background-color: var(--sidebar-hover);
  border-color: var(--accent-color);
}

/* 错误状态 */
.error-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
  background: var(--bg-secondary);
}

.error-content {
  text-align: center;
  max-width: 400px;
  padding: 2rem;
}

.error-content h2 {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.error-content h3 {
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.error-content p {
  color: var(--text-secondary);
  margin-bottom: 2rem;
}

.btn {
  background: var(--accent-color);
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  cursor: pointer;
  text-decoration: none;
  font-weight: 500;
  transition: all 0.3s ease;
}

.btn:hover {
  background-color: var(--accent-hover);
  transform: translateY(-1px);
}

/* 过渡动画 */
.slide-left-enter-active,
.slide-left-leave-active {
  transition: transform 0.3s ease;
}

.slide-left-enter-from {
  transform: translateX(100%);
}

.slide-left-leave-to {
  transform: translateX(100%);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .reader-header {
    padding: 0.75rem 1rem;
  }
  
  .book-info {
    margin: 0 1rem;
  }
  
  .book-info h2 {
    font-size: 1rem;
  }
  
  .settings-panel,
  .toc-panel {
    width: 280px;
  }
  
  .reader-footer {
    padding: 0.75rem 1rem;
    gap: 1rem;
  }
  
  .navigation-controls {
    flex-wrap: wrap;
  }
  
  .nav-btn {
    min-width: 60px;
    padding: 0.5rem 0.75rem;
    font-size: 0.8rem;
  }
}

@media (max-width: 640px) {
  .settings-panel,
  .toc-panel {
    width: 100vw;
  }
  
  .progress-info {
    flex-direction: column;
    gap: 0.25rem;
    min-width: 80px;
  }
  
  .theme-options {
    grid-template-columns: 1fr;
  }
}
</style>