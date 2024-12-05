# API Overview

### • Add a new newspaper
To add a new newspaper, provide the necessary information in JSON format. 

#### Example input
When sending the data, use the following JSON format. Note:

- The `signature` should begin with the Cyrillic letter "B", followed by four digits.
- If the newspaper is still being published or the end year is unknown, set the `end_year` field to `"null"`.

```json
{
  "CreateNewspaper": {
    "input": {
      "signature": "В1645",
      "name": "Стършел",
      "start_year": 1946,
      "end_year": null,
      "weekly_schedule": [
        false,
        false,
        false,
        false,
        true,
        false,
        false
      ]
    }
  }
}
```

#### Example request
```sh
curl -k -X POST https://dnevest.com/execute/dnevest \
 	-H "Content-Type: application/json" \
 	-d '{"CreateNewspaper":{"input":{"signature":"В1645","name":"Стършел","start_year":1946,"end_year":null,"weekly_schedule":[false, false, false, false, true, false, false]}}}'
```

#### Example response
```
{"offset_bytes":3451}
```

### • Find newspapers by date
To find newspapers published on a specific date, provide the date of interest in JSON format.

#### Example input
When specifying the date, use the following JSON format. Note:

- The `date` field must be in the format `"dd-mm-yyyy"`.

```json
{
  "NewspapersByDate": {
    "date": "16-08-2024"
  }
}
```

#### Example request
```sh
curl -k -X GET "https://dnevest.com/query/dnevest" \
  --get --data-urlencode '{"NewspapersByDate":{"date":"16-08-2024"}}'
```

#### Example response
```
[{"signature":"В1612","name":"Труд"},{"signature":"В1645","name":"Стършел"},{"signature":"В5499","name":"Стандарт"}]
```

### • Add a final year to mark the end of newspaper publication
To add a final year to an existing newspaper, provide the newspaper's `signature` and the `final_year` value in JSON format.

#### Example input
When specifying the signature and final year, use the following JSON format. Note:

- The `signature`  must match an existing newspaper.
- The `final_year` cannot be before the year of the first newspaper's publication.

```json
{
  "AddFinalYear": {
    "signature": "В1645",
    "final_year": 2024
  }
}
```

#### Example request
```sh
curl -k -X POST https://dnevest.com/execute/dnevest \
 	-H "Content-Type: application/json" \
 	-d '{"AddFinalYear":{"signature":"В1645","final_year":2024}}'
```

#### Example response
```
{"offset_bytes":2926}
```

### • Configure max number of cards
To set or update the max number of cards, provide the `max_number` value. If no value is stored, the system will store the provided number. 
Otherwise, it will update the stored number only if the new number is different from the existing one.

#### Example input
```json
{
  "MaxCardsConfig": {
    "max_number": 30
  }
}
```

#### Example request
```sh
curl -k -X POST https://dnevest.com/execute/dnevest \
 	-H "Content-Type: application/json" \
 	-d '{"SpecifyMaxCards":{"max_number":30}}'
```

### • Create a new order
To create a new order, provide the details for the order, the selected newspapers' signatures and the waybill information.

#### Example input
When specifing the properties, use the following JSON format.

- The `details` contains information about the appearance and content of the order. The `background` must contain a valid RGB value and the `card_id`
 must be within the maximum allowed value.
- The `newspapers` contains the ordered newspapers selected for the order. At least one newspaper must be selected and each `signature` must correspond 
to an existing newspaper.
- The `waybill` contains details about the customer and and the delivery address. The `customer_names` must include a minimum of 2 names and the `phone_number` 
must start with "0" or "+359" and contain exactly 9 digits after that.

```json
{
  "CreateOrder": {
    "order": {
      "details": {
        "background": [134, 24, 29],
        "frame": "White",
        "wish": "Честит рожден ден!",
        "font_type": "Times New Roman",
        "font_size": 12,
        "card_id": 10
      },
      "newspapers": ["В1633", "В1612", null],
      "waybill": {
        "customer_names": "Тодор Георгиев",
        "phone_number": "0873528495",
        "address": "Пловдив, ул.Тракия 12",
        "order_type": "Standart"
      }
    }
  }
}
```

#### Example request
```sh
curl -k -X POST https://dnevest.com/execute/dnevest \
 	-H "Content-Type: application/json" \
 	-d '{"CreateOrder":{"order":{"details":{"background":[134,24,29],"frame":"White","wish":"Честит рожден ден!","font_type":"Times New Roman","font_size":12,"card_id":10},"newspapers":["В1633","В1612",null],"waybill":{"customer_names":"Тодор Георгиев","phone_number":"0873528495","address":"Пловдив, ул.Тракия 12","order_type":"Standart"}}}}'
```