### Meowbot

```docker
docker run --restart=on-failure --name meowbot -d \
 -e bot_token=bot_token \
 -e admin_id=admin_id \
 meowdoublecat/meowbot:latest
```