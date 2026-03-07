Task Manager Program on Solana

Descripción del proyecto
Este proyecto implementa un programa en Solana utilizando Rust y Anchor que permite a los usuarios gestionar tareas almacenadas on-chain.
Cada usuario tiene una cuenta derivada por programa (PDA) donde se almacenan sus tareas, permitiendo operaciones CRUD como crear, actualizar, eliminar y visualizar tareas.

Tecnologías usadas
- Rust
- Solana
- Anchor Framework
- Program Derived Addresses (PDA)
- Phatom wallet

Arquitectura del proyecto 
TaskAccount 
- owner: wallet del usuario
- nombre_tablero: nombre del tablero de tareas
- tareas: vector de tareas almacenadas
Task
- titulo: nombre de la tarea
- completada: estado de la tarea

Tengo una app en la que he estado trabajando en Vue.js de tareas que me gustaría conectarla en un futuro. 
