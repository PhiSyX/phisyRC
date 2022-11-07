```text
<1-13 tokens> :are supported by this server
```

```text
token      =  *1"-" parameter / parameter *1( "=" value )
parameter  =  1*20 letter
value      =  * letpun
letter     =  ALPHA / DIGIT
punct      =  %d33-47 / %d58-64 / %d91-96 / %d123-126
letpun     =  letter / punct
```
