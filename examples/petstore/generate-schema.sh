#!/usr/bin/env bash

just run generate openapi generated
just run generate json-schema generated/schemas

JSON_DEFINITON=$(cat generated/openapi.json)

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
        const DEFINITION = ${JSON_DEFINITON};
        Redoc.init(DEFINITION, {}, document.getElementById('redoc-container'));
    </script>
  </body>
</html>
EOF

cat >generated/openapi-swagger-ui.html <<EOF
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="description" content="SwaggerUI" />

    <title>API Documentation</title>
    
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@latest/swagger-ui.css" />
  </head>
  <body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@latest/swagger-ui-bundle.js" crossorigin></script>
    <script>
      window.onload = () => {
        window.ui = SwaggerUIBundle({
          spec: ${JSON_DEFINITON},
          dom_id: '#swagger-ui',
        });
      };
    </script>
  </body>
</html>
EOF
