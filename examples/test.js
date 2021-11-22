import { Client } from '@std/http'

const response = (new Client)
    .get("https://jsonplaceholder.typicode.com/posts/1")
    .send()

println(response)