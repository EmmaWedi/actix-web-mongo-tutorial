var db = connect("mongodb://wedi:wedime@0.0.0.0:27017/dog_business");

db = db.getSiblingDB('dog_business'); // we can not use "use" statement here to switch db

db.createUser(
    {
        user: "wedi",
        pwd: "wedime",
        roles: [ { role: "readWrite", db: "dog_business"} ],
        passwordDigestor: "server",
    }
)