# json2redis
Simple rust project to take json from a file and stick it in a redis queue, for testing it's counterpart: redis2es

Whatever is in events.json will end up in redis in a queue called 'eventqueue', via threads, 10 entries at a time.

