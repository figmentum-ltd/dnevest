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
curl -k -X GET https://dnevest.com/query/dnevest \
 	-H "Content-Type: application/json" \
 	-d '{"NewspapersByDate":{"date":"16-08-2024"}}'
```

#### Example response
```
[{"signature":"В1645","name":"Стършел"}]
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
To set or update the max number of cards, provide the `max_number` value. If no value is stored, the system will store the provided number. Otherwise, it will update the stored number only if the new number is different from the existing one.

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