module Components.Models exposing (..)

import Material


-- MODEL


type alias Model =
    { count : Int
    , mdl :
        Material.Model

    -- Boilerplate: model store for any and all Mdl components you use.
    }


model : Model
model =
    { count = 0
    , mdl =
        Material.model

    -- Boilerplate: Always use this initial Mdl model store.
    }
