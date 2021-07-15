## Initialize the database


1. Requirements :
   - a steam installation of Age of Empire 2 DE.
   - rust + cargo
     
2. Create a development database (postgres) : 
```sql
CREATE USER scoutgg with password 'scoutgg';
CREATE DATABASE scoutgg WITH OWNER scoutgg;
```
3. Install and configure diesel cli : 
```shell
cargo install diesel_cli --no-default-features --features postgres
echo DATABASE_URL=postgres://scoutgg:scoutgg@localhost/scoutgg > .env
diesel setup
diesel migration run
diesel print-schema > src/schema.rs
```

4. clone [aoe2dat](https://github.com/HSZemi/aoe2dat) in the [scripts](scripts) folder. 
   ```shell
   git clone git@github.com:HSZemi/aoe2dat.git scripts/aoe2dat
   ```
   
5. Run the extraction script : 
```shell
cd scripts
./extract_game_data.sh ../resources/aoe2dat
```

6. Run scout binaries to populate the database 
```shell
cargo run --bin populate_db
cargo run --bin populate_tech_tree
```

## Usefull Aoe2DE Path

- Key value path
```shell
~/.steam/steam/steamapps/common/AoE2DE/resources/en/strings/key-value/
```
- Game save
```shell
~/.steam/steam/userdata/216344365/813780/remote/
```

- techthree 
```shell
~/.steam/steam/steamapps/common/AoE2DE/widgetui/civTechTrees.json
```