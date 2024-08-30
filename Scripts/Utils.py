def deal_timestamp(timestamp: int):
    timestamp = str(timestamp)
    return F'{timestamp[:4]}-{timestamp[4:6]}-{timestamp[6:8]}'
