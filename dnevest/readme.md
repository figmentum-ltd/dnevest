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
      "weekly_shedule": [
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
curl -k -X POST https://3.79.24.152/execute/dnevest \
 	-H "Content-Type: application/json" \
 	-d '{"CreateNewspaper":{"input":{"signature":"В1645","name":"Стършел","start_year":1946,"end_year":null,"weekly_shedule":[false, false, false, false, true, false, false]}}}'
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
curl -k -X GET https://3.79.24.152/execute/dnevest \
 	-H "Content-Type: application/json" \
 	-d '{"NewspapersByDate":{"date":"16-08-2024"}}'
```

#### Example response
```
[{"signature":"В1645","name":"Стършел"}]
```
