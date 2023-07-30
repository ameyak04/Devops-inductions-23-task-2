#Please login  user account with Sudo previledges.

sudo -i ( to Root) 

Assuming the home directory of root is /root 

#copy the files docker-compose-spidertask2.yaml, nginx.conf, package.json, and main.rs to the root directory

docker compose -f docker-compose-spidertask2.yaml up -d
#this should run all the servers

