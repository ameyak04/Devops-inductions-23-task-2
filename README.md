<h1>Spider Devops Inductions - Task 2 </h1>

<h2><b>Instructions after task completion:</b></h2>
Please login  user account with Sudo previledges.<br />

``` sudo -i ``` ( to Root) <br />

Assuming the home directory of root is /root <br />

copy the files docker-compose-spidertask2.yaml, nginx.conf, package.json, main.rs, diesel.sh and .env to the /root directory<br />

```mkdir scripts```<br />
```cd scripts```<br />
copy the file postgrescreatedb.sql to the /root/scripts directory<br />

```docker compose -f docker-compose-spidertask2.yaml up -d```<br />
this should run all the servers from /root directory<br />
refer <b>README.txt</b> for the detailed instructions<br />


<h2><b>Instructions before task completion:</b></h2>
1. Fork the given repo and set the your repo to <b>private</b>.<br />
2. Setup the servers using the instructions given below
3. Dockerize the given application using <b>Docker</b> and <b>Docker-compose</b><br />
<br/>
<hr/>
<h2><b>Tech Stack:</b></h2>
Language: <i>Rust</i><br />
Framework: <i>Actix-Web</i><br />
ORM: <i>Diesel</i><br />
Database: <i>PosgreSQL</i><br />
Frontend: <i>React</i><br />
Compilers: <i>Rust v1.7.0 and node v18.0.0</i><br />
<br/>
<hr/>

<b>How to Setup Server[Linux installation]:</b><br />
- ```cd Backend```<br />
- ```sudo apt install libpq-dev```
- ```cargo install diesel_cli --no-default-features --features postgres```
- CREATE DATABASE rust_server [in psql CLI];
- Add ENV variables for DB connection<br />
- ```diesel setup``` [Generates the tables]<br />
- ```cargo run``` [server runs]

<i>Any problems with setting up server refer to official cargo and diesel documents</i>

<br/>
<hr/>

<b>How to Setup Frontend React Server:</b><br />
- ```cd Frontend```<br />
- ```npm i``` [install dependencies]<br />
- ```npm start``` [start react server]<br />
