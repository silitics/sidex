#!/usr/bin/env bash

just run generate openapi generated
just run generate json-schema generated/schemas

OPENAPI_DEF=$(cat generated/openapi.json)

cat >generated/openapi-redoc.html <<EOF
<!DOCTYPE html>
<html>
  <head>
    <title>API Documentation</title>
    
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    
    <link href="https://fonts.googleapis.com/css?family=Montserrat:300,400,700|Roboto:300,400,700" rel="stylesheet">

    <style>
      body {
        margin: 0;
        padding: 0;
      }
    </style>
  </head>
  <body>
    <div id="redoc-container"></div>
    <script src="https://cdn.redoc.ly/redoc/latest/bundles/redoc.standalone.js"></script>
    <script>
        const DEFINITION = ${OPENAPI_DEF};
        Redoc.init(DEFINITION, {}, document.getElementById('redoc-container'));
    </script>
  </body>
</html>
EOF
