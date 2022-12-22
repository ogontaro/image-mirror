# image-mirror
[![sync repository to quay](https://github.com/ogontaro/image-mirror/actions/workflows/sync.yaml/badge.svg)](https://github.com/ogontaro/image-mirror/actions/workflows/sync.yaml)

## これは何？
コンテナイメージを[quay.io](https://quay.io/organization/image-mirror)にミラーリングしています

## なんでそんなことをしているの？
AWSのECRのプルスルーキャッシュの機能を利用して、イメージの脆弱性スキャンを行う環境を作りたいのですが、ECRのプルスルーキャッシュの対象のレジストリがECRとQuayしかないためです。
