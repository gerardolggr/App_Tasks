use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod Tareas{
    use super::*; 

    pub fn crear_tarea() -> Result<()>{
        //
    }
}

//Cuenta principal por Usuario que almacena sus tareas
pub struct TasksAccount{
    owner: Pubkey, 
    nombre_tablero: String, 
    tareas: Vec<Task>, 
}

//Estructura que almacena las tareas
pub struct Task{
    titulo: String,
    completada: bool, 
}

//Define las cuentas necesarias para la instrucción
pub struct NewTasksAccount{
    pub owner: Signer,
    pub task_account: Account<'info, TasksAccount>,
    pub system_program: Program<'info, System>, 
}
