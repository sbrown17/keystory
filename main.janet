(import spork/json)
(import redis)

(def conn (redis/connect))
  
(print "Connected to Redis!")

(defn main [& args]
  (print "Initializing Keystory..."))