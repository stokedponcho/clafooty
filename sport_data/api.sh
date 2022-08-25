#!/bin/sh

token=$(echo $API_SPORTDATA_AUTH_TOKEN)

#curl -X GET -H "apikey: $token" "https://app.sportdataapi.com/api/v1/soccer/leagues" | jq
curl -X GET -H "apikey: $token" "https://app.sportdataapi.com/api/v1/soccer/seasons?league_id=302" | jq
