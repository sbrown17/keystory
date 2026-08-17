(declare-project
  :name "Keystory"
  :description "An application for manipulating redis keys while syncing their state in git."
  :dependencies ["https://github.com/andrewchambers/janet-redis.git"])

(declare-executable
  :name "keystory"
  :entry "main.janet")