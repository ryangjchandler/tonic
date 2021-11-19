let name = "Ryan"
let age = 100
let truthy = true
let list = [name, age, truthy]

while (age < 150) {
    age = age + 1
}

function println(target) {
    console.log(`${target}`)
}

println(list)
println(age)
println(name)
println(truthy)

if (true == true) {
    println("True is true!")
}