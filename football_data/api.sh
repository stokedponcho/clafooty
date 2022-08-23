#!/bin/sh

token=$(echo $API_DATAFOOTBALL_AUTH_TOKEN)

#curl -X GET -H "X-Auth-Token: $token" http://api.football-data.org/v4/matches | jq
#curl -X GET -H "X-Auth-Token: $token" "http://api.football-data.org/v4/competitions/PL" | jq
curl -X GET -H "X-Auth-Token: $token" http://api.football-data.org/v4/competitions/PL/matches?matchday=6 | jq
#curl -X GET -H "X-Auth-Token: $token" http://api.football-data.org/v4/competitions/2021/standings | jq
