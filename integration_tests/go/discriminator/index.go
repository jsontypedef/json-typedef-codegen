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



type Discriminator struct {
  Version Version `json:"version"`

  V1 `json:"-"`

  V2 `json:"-"`

}

func (d Discriminator) MarshalJSON() ([]byte, error) {
  switch d.Version {

  case "v1":
    return json.Marshal(struct { Tag string `json:"version"`; V1 }{ Tag: "v1", V1: d.V1 })

  case "v2":
    return json.Marshal(struct { Tag string `json:"version"`; V2 }{ Tag: "v2", V2: d.V2 })

  default:
    panic("asdf")
  }
}

func (d *Discriminator) UnmarshalJSON(b []byte) error {
  var base struct { Tag string `json:"version"` }
  if err := json.Unmarshal(b, &base); err != nil {
    return err
  }

  switch base.Tag {

  case "v1":
    d.Version = "v1"
    return json.Unmarshal(b, &d.V1)

  case "v2":
    d.Version = "v2"
    return json.Unmarshal(b, &d.V2)

  default:
    panic("asdf")
  }
}
