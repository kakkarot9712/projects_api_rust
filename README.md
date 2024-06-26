# Note
The [Portfolio](https://vikalpg.in) website is not using this backend anymore as the Codebase of the portfolio is switched to NextJS and all basic APIs are integrated into Portfolio Projects itself.

# Portfolio API
Simple API to retrieve stored portfolio metadata from database using supported endpoints, created using Rust, Actix, MongoDB for the database.

# Supported Endpoints

## `GET` Languages

_Returns the list of metadata of Programming Languages I know along with base64 SVG data._

- **URL**
  `/languages`

- **Method**
  `GET`
- **Success Response**
  - **Code:** 200 <br />
    **Content:** `[ /* List of Language Metadata */]`

## `GET` Tools

_Gets the list of metadata of Tools I used and am familiar with, Like Render, which was once used for deployment of this web server along with base64 SVG data._

- **URL**
  `/tools`
- **Method**
  `GET`
- **Success Response**
  - **Code:** 200 <br />
    **Content:** `[ /* List of Tools Metadata */]`

<!--
## `GET` Projects

_Gets the list of metadata of Projects I made for practice and learning purposes._
* **URL**
  `/projects`

* **Method**
  `GET`

* **Success Response**
  * **Code:** 200 <br />
    **Content:** `{"status": "success", "data": {"projects": [ /* List of Projects Metadata */] }}`

* **Error Response**
  * **Code:** 500 <br />
    **Content:** `{
      "status": "failed",
      "message": "Something Went Wrong",
    }` -->
