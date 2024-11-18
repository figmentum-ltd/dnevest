#!/bin/bash

SERVER_DIR="https://figmentum.net/execute/dnevest"
CONTENT_TYPE="application/json"

create_newspaper() {
  local signature="$1"
  local name="$2"
  local start_year="$3"
  local end_year="$4"
  local weekly_schedule="$5"

  curl -k -X POST "$SERVER_DIR" \
    -H "Content-Type: $CONTENT_TYPE" \
    -d "{\"CreateNewspaper\":{\"input\":{\"signature\":\"$signature\",\"name\":\"$name\",\"start_year\":$start_year,\"end_year\":$end_year,\"weekly_schedule\":$weekly_schedule}}}"
}

create_newspaper "В1645" "Стършел" 1946 null "[false, false, false, false, true, false, false]"
create_newspaper "В1602" "Работническо дело" 1945 1989 "[true, true, true, true, true, true, true]"
create_newspaper "В1637" "Народна младеж" 1945 1989 "[true, true, true, true, true, true, true]"
create_newspaper "В1633" "Литературен фронт" 1945 1993 "[false, false, false, true, false, false, false]"
create_newspaper "В1612" "Труд" 1946 null "[true, true, true, true, true, true, true]"
create_newspaper "В4667" "Орбита" 1969 1991 "[false, false, false, false, false, true, false]"
create_newspaper "В1616" "Народен спорт" 1944 1989 "[true, false, false, true, false, true, false]"
create_newspaper "В5056" "Дума" 1990 1998 "[true, true, true, true, true, true, true]"
create_newspaper "В5057" "Демокрация" 1990 2002 "[true, true, true, true, true, true, true]"
create_newspaper "В1708" "Вечерни новини" 1952 1992 "[true, true, true, true, true, true, true]"
create_newspaper "В5499" "Стандарт" 1992 null "[true, true, true, true, true, true, true]"
create_newspaper "В1905" "Поглед" 1966 1996 "[true, false, false, false, false, false, false]"
create_newspaper "В1621" "Отечествен глас" 1960 1989 "[true, true, true, true, true, true, true]"
create_newspaper "В1601" "Отечествен фронт" 1945 1989 "[false, true, true, true, true, true, true]"