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
DB:db Dockerfile downloads official postgres image from dockerhub sets password and also run this sql script at the start



Frontend:frontend dockerfile sets up the frontend server
FROM ubuntu:latest
RUN apt update && apt-get install -y npm
RUN git clone https://github.com/ameyak04/Devops-inductions-23-task-2.git /root/Devops-inductions-23-task-2
COPY ./package.json /root/Devops-inductions-23-task-2/Frontend
WORKDIR /root/Devops-inductions-23-task-2/Frontend
RUN npm i
ENTRYPOINT npm start



backend dockerfile sets up the backend server
FROM ubuntu:latest
ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/root/.cargo/bin
RUN apt update && apt-get install -y git cargo libpq-dev
RUN git clone https://github.com/ameyak04/Devops-inductions-23-task-2.git /root/Devops-inductions-23-task-2
COPY ./diesel.sh /root/Devops-inductions-23-task-2/Backend/diesel.sh
COPY ./.env /root
WORKDIR /root/Devops-inductions-23-task-2/Backend
RUN apt install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres
RUN chmod 777 /root/Devops-inductions-23-task-2/Backend/diesel.sh
ENTRYPOINT /root/Devops-inductions-23-task-2/Backend/diesel.sh


to check if the website is running --->
http://localhost:3000/

to create User ---->
http://localhost:3000/register






