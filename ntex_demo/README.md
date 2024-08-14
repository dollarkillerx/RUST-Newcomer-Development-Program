
### table 

``` 
CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    create_date DATE NOT NULL
);

CREATE TABLE oauth_user (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    avatar_url VARCHAR(255) NOT NULL,
    create_date DATE NOT NULL
);
```

``` 
- ntex
- env_logger
- sqlx
- chrono time
- serde serde_json
- dotenvy
```


``` 
https://docs.github.com/zh/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps
https://github.com/login/oauth/authorize?client_id=Ov23ct6IBNGJ7yHCsNNy


```