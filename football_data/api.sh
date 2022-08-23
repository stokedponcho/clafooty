#!/bin/sh

token=$(echo $API_DATAFOOTBALL_AUTH_TOKEN)

#curl -X GET -H "X-Auth-Token: $token" "http://api.football-data.org/v2/competitions/PL"
#curl -X GET -H "X-Auth-Token: $token" http://api.football-data.org/v2/competitions?plan=TIER_ONE | jq
#curl -X GET -H "X-Auth-Token: $token" http://api.football-data.org/v2/competitions/2146/matches
#curl -X GET -H "X-Auth-Token: $token" http://api.football-data.org/v2/competitions/PL/matches?matchday=6
#curl -X GET -H "X-Auth-Token: $token" "http://api.football-data.org/v2/competitions/PL/matches?Status=SCHEDULED&dateFrom=2020-10-20"
#curl -X GET -H "X-Auth-Token: $token" "http://api.football-data.org/v2/competitions/2021/matches?dateFrom=2020-10-20&dateTo=2020-10-27"
#curl -X GET -H "X-Auth-Token: $token" "http://api.football-data.org/v2/competitions/FL1/matches?matchday=11"
#curl -X GET -H "X-Auth-Token: $token" "http://api.football-data.org/v2/competitions/PL/matches?dateFrom=2020-10-20&dateTo=2020-10-27"
#curl -X GET -H "X-Auth-Token: $token" "http://api.football-data.org/v2/competitions/FL1/matches?dateFrom=2020-10-20&dateTo=2020-10-27"
#curl -X GET -H "X-Auth-Token: $token" http://api.football-data.org/v2/teams/73/matches
#curl -X GET -H "X-Auth-Token: $token" http://api.football-data.org/v2/competitions/2021/standings | jq
curl -X GET -H "X-Auth-Token: $token" http://api.football-data.org/v2/matches | jq
