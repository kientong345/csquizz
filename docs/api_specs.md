# API Specifications

**Base URL:** `/api`

> [!NOTE]
> **Case Convention:**
> - **Requests** (Input): Use `camelCase` for JSON property names (DTOs).
> - **Responses** (Output): Use `camelCase` for JSON property names.
> - **Exceptions**: Nested JSON objects like `key` (in questions) and `data` (in answers) use `snake_case` as they map directly to database JSON structures.

---

## 1. Authentication (`/auth`)

### 1.1. Register
- **Endpoint:** `POST /api/auth/register`
- **Description:** Register a new user.
- **Request Body:** `application/json`
  ```json
  {
    "displayName": "string",
    "email": "string",
    "password": "string"
  }
  ```
- **Success Response:** `201 Created`

### 1.2. Login
- **Endpoint:** `POST /api/auth/login`
- **Description:** Authenticate user and receive access token. Refresh token is set in HttpOnly cookie.
- **Request Body:** `application/json`
  ```json
  {
    "email": "string",
    "password": "string"
  }
  ```
- **Success Response:** `200 OK`
  ```json
  {
    "accessToken": "string"
  }
  ```
  **Cookies:** `refresh_token=...; HttpOnly; Path=/`

### 1.3. Google Login
- **Endpoint:** `POST /api/auth/google-login`
- **Description:** Exchange Google Authorization Code for access token.
- **Query Parameters:**
  - `code`: `string` (The authorization code from Google)
- **Success Response:** `200 OK`
  ```json
  {
    "accessToken": "string"
  }
  ```
  **Cookies:** `refresh_token=...; HttpOnly; Path=/`

### 1.4. Logout
- **Endpoint:** `POST /api/auth/logout`
- **Description:** Log out the user (invalidate refresh token).
- **Success Response:** `200 OK` (Implementation pending)

### 1.5. Refresh Token
- **Endpoint:** `POST /api/auth/refresh`
- **Description:** Get a new access token using the refresh token cookie.
- **Success Response:** `200 OK` (Implementation pending)

---

## 2. Quizzes (`/quizzes`)

### 2.1. Get Quizzes Page
- **Endpoint:** `GET /api/quizzes`
- **Description:** Get a paginated list of quizzes.
- **Query Parameters:**
  - `page`: `number` (optional)
  - `page_size`: `number` (optional)
  - `title_pattern`: `string` (optional)
  - `category_id`: `number` (optional)
  - `difficulty`: `string` (optional)
- **Success Response:** `200 OK`
  ```json
  {
    "items": [
      {
        "id": 1,
        "title": "Quiz Title",
        "description": "Description",
        "difficulty": "easy",
        "categoryId": 1,
        "creatorId": 1,
        "passScore": 50.0,
        "createdAt": "timestamp",
        "updatedAt": "timestamp",
        "questionCount": 10,
        "likeCount": 5,
        "categoryName": "Category Name"
      }
    ],
    "totalItems": 100,
    "totalPages": 10
  }
  ```

### 2.2. Get Quiz with Questions
- **Endpoint:** `GET /api/quizzes/{id}/questions`
- **Description:** Get a quiz details along with its questions.
- **Query Parameters:**
  - `page`: `number`
  - `page_size`: `number`
- **Success Response:** `200 OK`
  ```json
  {
    "quiz": {
      "id": 1,
      "title": "Quiz Title",
      "description": "Description",
      "difficulty": "easy",
      "categoryId": 1,
      "creatorId": 1,
      "passScore": 50.0,
      "createdAt": "timestamp",
      "updatedAt": "timestamp",
      "questionCount": 10,
      "likeCount": 5,
      "commentCount": 2,
      "categoryName": "Category Name"
    },
    "questions": {
      "items": [
        {
          "id": 1,
          "type": "multiple_choice", // or "single_choice", "text_entry"
          "content": "Question text",
          "imageUrl": "url",
          "publicData": { ... }, // Options for choice questions (see Data Structures)
          "quizId": 1,
          "createdAt": "timestamp"
        }
      ],
      "totalItems": 10,
      "totalPages": 1
    }
  }
  ```

### 2.3. Get Quiz Comments
- **Endpoint:** `GET /api/quizzes/{id}/comments`
- **Description:** Get comments for a quiz.
- **Query Parameters:**
  - `page`: `number`
  - `page_size`: `number`
  - `sort_by`: `string`
- **Success Response:** `200 OK`

### 2.4. Create Quiz (with Questions)
- **Endpoint:** `POST /api/quizzes`
- **Authentication:** Required
- **Request Body:** `application/json`
  ```json
  {
    "quizParams": {
      "title": "string",
      "description": "string", // optional
      "difficulty": "string", // optional
      "categoryId": 1,
      "passScore": 50.0
    },
    "questionsParams": [
      {
        "type": "multiple_choice", // "single_choice", "text_entry"
        "content": "string",
        "imageUrl": "string", // optional
        "key": { ... } // See Question Key Structure below (snake_case)
      }
    ]
  }
  ```
- **Success Response:** `201 Created`

### 2.5. Like Quiz
- **Endpoint:** `POST /api/quizzes/{id}/like`
- **Authentication:** Required
- **Success Response:** `201 Created`

### 2.6. Comment on Quiz
- **Endpoint:** `POST /api/quizzes/{id}/comment`
- **Authentication:** Required
- **Request Body:**
  ```json
  {
    "content": "string"
  }
  ```
- **Success Response:** `201 Created`

### 2.7. Submit Quiz
- **Endpoint:** `POST /api/quizzes/{id}/submit`
- **Authentication:** Required
- **Request Body:**
  ```json
  {
    "answersParams": [
      {
        "questionId": 1,
        "data": { ... } // See Answer Data Structure below (snake_case)
      }
    ]
  }
  ```
- **Success Response:** `200 OK`

### 2.8. Add Question to Quiz
- **Endpoint:** `POST /api/quizzes/{id}/questions`
- **Authentication:** Required (Owner)
- **Request Body:** `QuestionCreateParamsDto` (same as in Create Quiz)
- **Success Response:** `201 Created`

### 2.9. Update Question
- **Endpoint:** `PATCH /api/quizzes/{id}/questions/{question_id}`
- **Authentication:** Required (Owner)
- **Request Body:**
  ```json
  {
    "type": "string", // optional
    "content": "string", // optional
    "imageUrl": "string", // optional
    "key": { ... } // optional
  }
  ```
- **Success Response:** `200 OK`

### 2.10. Delete Question
- **Endpoint:** `DELETE /api/quizzes/{id}/questions/{question_id}`
- **Authentication:** Required (Owner)
- **Success Response:** `200 OK`

### 2.11. Update Quiz Metadata
- **Endpoint:** `PATCH /api/quizzes/{id}`
- **Authentication:** Required (Owner)
- **Request Body:**
  ```json
  {
    "title": "string", // optional
    "description": "string", // optional
    "difficulty": "string", // optional
    "categoryId": 1, // optional
    "passScore": 50.0 // optional
  }
  ```
- **Success Response:** `200 OK`

### 2.12. Delete Quiz
- **Endpoint:** `DELETE /api/quizzes/{id}`
- **Authentication:** Required (Owner)
- **Success Response:** `200 OK`

---

## 3. Categories (`/categories`)

### 3.1. Get Categories Page
- **Endpoint:** `GET /api/categories`
- **Query Parameters:**
  - `page`: `number`
  - `page_size`: `number`
  - `sort_by`: `string`
- **Success Response:** `200 OK`
  ```json
  {
    "items": [
      {
        "id": 1,
        "name": "string",
        "imageUrl": "string",
        "description": "string"
      }
    ],
    "totalItems": 10,
    "totalPages": 1
  }
  ```

### 3.2. Get All Categories
- **Endpoint:** `GET /api/categories/all`
- **Description:** Get all categories without pagination.
- **Success Response:** `200 OK`

### 3.3. Get Category by ID
- **Endpoint:** `GET /api/categories/{id}`
- **Success Response:** `200 OK`
  ```json
  {
    "id": 1,
    "name": "string",
    "imageUrl": "string",
    "description": "string"
  }
  ```

### 3.4. Create Category
- **Endpoint:** `POST /api/admin/categories`
- **Authentication:** Admin
- **Request Body:**
  ```json
  {
    "name": "string",
    "imageUrl": "string", // optional
    "description": "string" // optional
  }
  ```
- **Success Response:** `201 Created`

### 3.5. Update Category
- **Endpoint:** `PATCH /api/admin/categories/{id}`
- **Authentication:** Admin
- **Request Body:**
  ```json
  {
    "name": "string", // optional
    "imageUrl": "string", // optional
    "description": "string" // optional
  }
  ```
- **Success Response:** `200 OK`

### 3.6. Delete Category
- **Endpoint:** `DELETE /api/admin/categories/{id}`
- **Authentication:** Admin
- **Success Response:** `200 OK`

---

## 4. Users (`/users`)

### 4.1. Get Users Page
- **Endpoint:** `GET /api/users`
- **Query Parameters:**
  - `page`: `number`
  - `page_size`: `number`
  - `sort_by`: `string`
- **Success Response:** `200 OK`
  ```json
  {
    "items": [
      {
        "id": 1,
        "displayName": "string",
        "avatarUrl": "string"
      }
    ],
    "totalItems": 10,
    "totalPages": 1
  }
  ```

### 4.2. Get User by ID
- **Endpoint:** `GET /api/users/{id}`
- **Success Response:** `200 OK`
  ```json
  {
    "id": 1,
    "displayName": "string",
    "avatarUrl": "string",
    "createdAt": "timestamp",
    "quizCompletedCount": 10,
    "quizCreatedCount": 5,
    "followerCount": 2
  }
  ```

### 4.3. Get Current User (Me)
- **Endpoint:** `GET /api/users/me`
- **Authentication:** Required
- **Success Response:** `200 OK`
  ```json
  {
    "id": 1,
    "displayName": "string",
    "email": "string",
    "avatarUrl": "string",
    "role": "user",
    "createdAt": "timestamp",
    "quizCompletedCount": 10,
    "quizCreatedCount": 5,
    "followerCount": 2
  }
  ```

### 4.4. Update Current User
- **Endpoint:** `PATCH /api/users/me`
- **Authentication:** Required
- **Request Body:**
  ```json
  {
    "displayName": "string", // optional
    "passwordHash": "string", // optional
    "avatarUrl": "string" // optional
  }
  ```
- **Success Response:** `200 OK`

### 4.5. Get My Submissions
- **Endpoint:** `GET /api/users/me/submissions`
- **Authentication:** Required
- **Query Parameters:**
  - `quiz_title_pattern`: `string`
  - `passed_only`: `boolean`
  - `quiz_difficulty`: `string` (optional)
  - `page`: `number`
  - `page_size`: `number`
  - `sort_by`: `string`
- **Success Response:** `200 OK`
  ```json
  {
    "items": [
      {
        "id": 1,
        "score": 80.0,
        "isPassed": true,
        "submittedAt": "timestamp",
        "quizTitle": "Quiz Title"
      }
    ],
    "totalItems": 10,
    "totalPages": 1
  }
  ```

### 4.6. Update User (Admin)
- **Endpoint:** `PATCH /api/admin/users/{id}`
- **Authentication:** Admin
- **Request Body:** Same as Update Current User
- **Success Response:** `200 OK`

### 4.7. Delete User (Admin)
- **Endpoint:** `DELETE /api/admin/users/{id}`
- **Authentication:** Admin
- **Success Response:** `200 OK`

---

## 5. Admin (`/admin`)

### 5.1. Grant Admin Permission
- **Endpoint:** `PUT /api/admin/grant`
- **Authentication:** Admin (Likely)
- **Description:** Placeholder endpoint.
- **Success Response:** `200 OK`

---

## Data Structures

### Question Key Structure (`key`)

**Multiple Choice / Single Choice:**
```json
{
  "keys": [
    {
      "id": 1,
      "content": "Option text",
      "image_url": "url", // snake_case (nested object)
      "is_correct": true,
      "explanation": "Why this is correct"
    }
  ]
}
```

**Text Entry:**
```json
{
  "correct_entry": "exact answer",
  "explanation": "Explanation"
}
```

### Answer Data Structure (`data`)

**Multiple Choice / Single Choice:**
```json
{
  "choices": [
    {
      "option_id": 1 // snake_case (nested object)
    }
  ]
}
```

**Text Entry:**
```json
{
  "entry": "user answer"
}
```