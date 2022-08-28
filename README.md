# Car API

Written in Rust & Rocket.rs 🦀🚀

---

## Running project

#### Running by cargo

You just need to run following command:

```
cargo run
```

#### Running on docker

First of all you need to build image from *Dockerfile* by running: 

```
docker build -t your_image_name .
```

> **Note** you can replace *your_image_name* by whatever name you want the image of app to be

To start image and create a container  

```
docker run --name your_container_name -p 8000:8000 your_image_name
```

> **Note** you can replace *your_container_name* by whatever name you want the container to be

---

©bartekmolka 2022
