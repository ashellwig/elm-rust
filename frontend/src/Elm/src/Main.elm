module Main exposing (..)

import Components.Models exposing (..)
import Components.Types exposing (..)
import Components.Update exposing (..)
import Components.Views exposing (..)
import Html exposing (program)


main : Program Never Model Msg
main =
    Html.program
        { init = ( model, Cmd.none )
        , view = view
        , subscriptions = always Sub.none
        , update = update
        }
