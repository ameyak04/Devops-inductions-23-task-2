#Please login  user account with Sudo previledges.

sudo -i ( to Root) 

Assuming the home directory of root is /root 

#copy the files docker-compose-spidertask2.yaml, nginx.conf, package.json, main.rs, diesel.sh and .env to the /root directory

mkdir scripts
cd scripts
#copy the file postgrescreatedb.sql to the /root/scripts directory

docker compose -f docker-compose-spidertask2.yaml up -d
#this should run all the servers from /root directory

#Dockerfiles are uploaded for reference ---> the images used in the compose file are built on these dockerfiles
db Dockerfile downloads official postgres image from dockerhub sets password and also run this sql script at the start
frontend dockerfile sets up the frontend server
backend dockerfile sets up the backend server
