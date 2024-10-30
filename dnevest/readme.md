# API Overview

### • Add a new newspaper
In the process of adding a new newspaper, the `execute()` command is used, providing it with the necessary information to create the newspaper. The input data must be in JSON format. 

#### Example input
When calling the `execute()`, send the data in the following JSON format. Note:

- The `signature` should begin with the Cyrillic letter "B", followed by four digits.
- If the newspaper is still being published or the end year is unknown, set the `end_year` field to `"null"`.

```json
{
  "CreateNewspaper": {
    "input": {
      "signature": "В4667",
      "name": "Орбита",
      "start_year": 1969,
      "end_year": 1991,
      "weekly_shedule": [
        false,
        false,
        false,
        false,
        false,
        true,
        false
      ]
    }
  }
}
```


### • Find newspapers by date
The process of searching for newspapers published on a particular date uses the `query()` method. The provided data must be in JSON format.

#### Example input
When calling `query()`, send the data in the following JSON format. Note:

- The `date` field must be in the format `"dd-mm-yyyy"`.

```json
{
  "NewspapersByDate": {
    "date": "29-06-2024"
  }
}
```
if successful, the `query()`method returns the serialized newspapers where every newspaper is presented only by its `signature` and `name`.
