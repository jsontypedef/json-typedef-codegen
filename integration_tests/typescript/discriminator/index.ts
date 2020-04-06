
export type Discriminator = V1 | V2;


export interface V1User {

    favoriteNumbers: number[];

    id: string;
}


export interface V1 {

    user: V1User;

    version: "v1";
}


export interface V2User {

    favoriteNumbers: string[];

    id: string;
}


export interface V2 {

    user: V2User;

    version: "v2";
}

