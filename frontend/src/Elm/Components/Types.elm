module Components.Types exposing (..)

import Components.Models exposing (..)
import Material


-- ACTION, UPDATE


type Msg
    = Increase
    | Reset
    | Mdl (Material.Msg Msg)
