<template>
  <Teleport to="body">
    <div v-if="show" class="modal-overlay" @click="handleOverlayClick">
      <BaseCard 
        class="modal-container" 
        :class="{ 'modal-large': size === 'large', 'modal-small': size === 'small' }"
      >
        <template #header v-if="title || $slots.header">
          <div class="modal-header">
            <slot name="header">
              <h3 class="modal-title">{{ title }}</h3>
            </slot>
            <BaseButton
              v-if="closable"
              @click="close"
              variant="ghost"
              size="small"
              icon="✕"
            />
          </div>
        </template>
        
        <div class="modal-body">
          <slot></slot>
        </div>
        
        <template #footer v-if="$slots.footer">
          <div class="modal-footer">
            <slot name="footer"></slot>
          </div>
        </template>
      </BaseCard>
    </div>
  </Teleport>
</template>

<script>
import { watch } from 'vue'

export default {
  name: 'Modal',
  props: {
    show: {
      type: Boolean,
      default: false
    },
    title: {
      type: String,
      default: ''
    },
    size: {
      type: String,
      default: 'medium',
      validator: (value) => ['small', 'medium', 'large'].includes(value)
    },
    closable: {
      type: Boolean,
      default: true
    },
    closeOnOverlay: {
      type: Boolean,
      default: true
    }
  },
  emits: ['close', 'update:show'],
  setup(props, { emit }) {
    const close = () => {
      emit('close')
      emit('update:show', false)
    }

    const handleOverlayClick = (event) => {
      if (props.closeOnOverlay && event.target === event.currentTarget) {
        close()
      }
    }

    const handleEscape = (event) => {
      if (event.key === 'Escape' && props.show && props.closable) {
        close()
      }
    }

    watch(() => props.show, (newValue) => {
      if (newValue) {
        document.addEventListener('keydown', handleEscape)
        document.body.style.overflow = 'hidden'
      } else {
        document.removeEventListener('keydown', handleEscape)
        document.body.style.overflow = ''
      }
    })

    return {
      close,
      handleOverlayClick
    }
  }
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 1rem;
}

.modal-container {
  max-width: 500px;
  width: 100%;
  max-height: 90vh;
  overflow: hidden;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
}

.modal-small {
  max-width: 300px;
}

.modal-large {
  max-width: 800px;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.modal-title {
  margin: 0;
  color: var(--text-primary);
  font-size: 1.25rem;
  font-weight: 600;
}

.modal-close {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #6c757d;
  padding: 0.25rem;
  border-radius: 4px;
  transition: all 0.3s ease;
}

.modal-close:hover {
  background-color: #f8f9fa;
  color: #495057;
}

.modal-body {
  padding: 1.5rem;
  overflow-y: auto;
  flex: 1;
}

.modal-footer {
  padding: 1rem 1.5rem;
  border-top: 1px solid #e9ecef;
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
}

/* 响应式设计 */
@media (max-width: 640px) {
  .modal-container {
    margin: 1rem;
    max-width: none;
    width: calc(100% - 2rem);
  }
  
  .modal-header,
  .modal-body,
  .modal-footer {
    padding: 1rem;
  }
}
</style>