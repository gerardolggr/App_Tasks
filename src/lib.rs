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
#[account]
#[derive(InitSpace)]
pub struct TasksAccount{
    owner: Pubkey,

    #[max_len(60)] 
    nombre_tablero: String, 

    #[max_len(10)]
    tareas: Vec<Task>, 
}

//Estructura que almacena las tareas
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Task{
    #[max_len(60)]
    titulo: String,

    completada: bool, 
}

//Define lo que necesita una cuenta para ejecutar la instrucción
#[derive(Accounts)]
pub struct NewTasksAccount{
    #[account(mut)]
    pub owner: Signer<'info>, 

    #[account(
        init, payer = owner, 
        space = TasksAccount::INIT_SPACE + 8, 
        seeds = [b"Tareas", 
        owner.key().as_ref(), 
        bump])]
    pub task_account: Account<'info, TasksAccount>,


    pub system_program: Program<'info, System>, 
}

//Estructura que ayuda a dar las instrucciones (CRUD) de la cuenta ya creada
pub struct UpdateTask{
    pub owner: Signer<'info>,

    #[account(mut)]
    pub task_account: Account<'info, TasksAccount>, 
}
