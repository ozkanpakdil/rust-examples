## generate jwt

```shell
curl -X POST "http://localhost:8080/signin" \
-H "Content-Type: application/json" \
-d '{
"email": "myemail@gmail.com",
"password": "password"
}'
```

## access protected endpoint
```shell
curl -X GET http://localhost:8080/protected/ \
-H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NDEyMDkwMTAsImlhdCI6MTc0MTEyMjYxMCwiZW1haWwiOiJteWVtYWlsQGdtYWlsLmNvbSJ9.Wa2hg9LJePcRy8utytWWibUImpYOKbfxeqqKegeDleo" \
-H "Content-Type: application/json" 
```