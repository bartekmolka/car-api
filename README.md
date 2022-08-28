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
> **Note** If you're running on windows you have to open *127.0.0.1:8000* instead of *0.0.0.0:8000*
---

©bartekmolka 2022
