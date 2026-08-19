#(import spork/json)
#(import redis)
(import janetui)

(def conn (redis/connect))
  
(print "Connected to Redis!")

(defn main [& args]
  (print "Initializing Keystory...")
  (def window (janetui/new-window "keystory" 500 300 1))
)