package properties


import "time"




type D struct {

  A uint32 `json:"a"`
}


type Properties struct {

  A *string `json:"a"`

  B time.Time `json:"b"`

  C *string `json:"c"`

  D *D `json:"d"`
}


