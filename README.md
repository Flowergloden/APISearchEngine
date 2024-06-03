# API Search Engine

## 依赖
### 前端
- dotnet 8.0
- Uno.Check (dotnet tool)

### 后端
- Rust 工具链
- docker

## 构建
### 前端
在 Client 文件夹下运行：
`dotnet publish -f net8.0-browserwasm -c Release -o ./publish`

### 后端
在 server 文件夹下运行：
`docker image build -t server .`

## 部署
### 前端
将构建文件夹下的 wwwroot 目录用 Web 服务器提供服务即可。

### 后端
运行容器即可：
`docker container run -dp 8000:8000 -t server`

## 使用
访问前端对应的地址即可使用。