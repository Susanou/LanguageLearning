#!/bin/bash

var="8dm7KsjU28B7v621Jls"
value="ERmFRMVZ0U2paTlJYTkxDZz09Cg"

for i in {1..40}
do
        var=$(echo $var | base64)
		
		#<---- If condition here:
        if [[ "$var" == *"$value"* ]] && [ $(echo $var | wc -c) -gt 113469 ]; then
        echo $var > test
            tail -c 20 test
        fi
done
