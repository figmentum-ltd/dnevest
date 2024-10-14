# Architecture Decision Record (ADR)

## ADR-001: Choice of Rendering

**Date:** 2024-10-01  
**Status:** Accepted  
**Decision:**  
We will use client-side rendering. 

**Description:**
- CSR enables a more dynamic and interactive user experience since the application updates and re-renders components in real-time based on user actions.

---

## ADR-002: Frontend Framework Selection

**Date:** 2024-10-08  
**Status:** Proposed  
**Decision:**  
For the frontend we will use React or VueJS

**Description:**  
- We decided to use one of these platforms because they have a lot of ready-made components, it will be easier to make the frontend, as opposed to the yew.

---

## ADR-003: Message Format Selection

**Date:** 2024-10-08  
**Status:** Accepted  
**Decision:**  
We have chosen **JSON** as the standard format for the messages exchanged between frontend and backend.

**Description:**  
- It is human-readable and widely supported across different languages and platforms, making it easier to integrate with various systems.
- JSON has broad compatibility with frontend frameworks like React and Vue, as well as backend technologies.
- Parsing JSON is fast and well-supported in most environments, which helps with efficient data processing.

---

## ADR-004: Selecting newspapers from web

**Date:** 2024-10-08  
**Status:** Accepted  
**Decision:**   
For the customer, the name and the signature of the newspapers will be visible and through them they will be able to select the newspapers.


