# Car API

Written in Rust & Rocket.rs 🦀🚀

---

## Running project

#### Running by cargo

You just need to run the following command:

```
cargo run
```

#### Running on docker

First of all you need to build image from *Dockerfile* by running: 

```
docker build -t your_image_name .
```

> **Note** you can replace *your_image_name* by whatever name you want the image of app to be

To start image and create a container type:

```
docker run --name your_container_name -p 8000:8000 your_image_name
```

> **Note** you can replace *your_container_name* by whatever name you want the container to be

Then you can open app on for example *[localhost:8000](http://localhost:8000/probabilityOfUnitInjectorFail?vin=WDBRF40J43F433102)*
---

©bartekmolka 2022
