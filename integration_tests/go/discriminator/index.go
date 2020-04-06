package discriminator


import "encoding/json"



type Version string

const VersionV1 Version = "v1"

const VersionV2 Version = "v2"


type V1 struct {

  User V1User `json:"user"`
}


type V1User struct {

  FavoriteNumbers []uint32 `json:"favoriteNumbers"`

  Id string `json:"id"`
}


type V2 struct {

  User V2User `json:"user"`
}


type V2User struct {

  FavoriteNumbers []string `json:"favoriteNumbers"`

  Id string `json:"id"`
}


