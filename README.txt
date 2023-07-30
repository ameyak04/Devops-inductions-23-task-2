#Please login  user account with Sudo previledges.

sudo -i ( to Root) 

Assuming the home directory of root is /root 

#copy the files docker-compose-spidertask2.yaml, nginx.conf, package.json, main.rs, diesel.sh and .env to the /root directory

mkdir scripts
cd scripts
#copy the file postgrescreatedb.sql to the /root/scripts directory

docker compose -f docker-compose-spidertask2.yaml up -d
#this should run all the servers

#Dockerfiles are uploaded for reference ---> the images used in the compose file are built on these dockerfiles
db Dockerfile 
