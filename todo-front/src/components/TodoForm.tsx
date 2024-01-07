import {FC, useState} from 'react'
import { NewTodoPayload } from '../types/todo'
import { Box, Button, TextField, Paper, Grid} from '@mui/material'

// point 1
type Prosp = {
    onSubmit: (newTodo: NewTodoPayload) => void
}

const TodoForm: FC<Prosp> = ({onSubmit}) => {
    // point2
    const [editText, setEditText] = useState('')

    // point3
    const addTodoHandler = async () => {
        if (!editText)  return

        onSubmit({
            text: editText,
        })
        setEditText('')
    }

    return (
        <Paper elevation={2}>
            <Box sx = {{ p: 2 }}>
                <Grid container spacing={2} columnSpacing={5}>
                    <Grid item xs={12}>
                        <TextField
                            label="new Todo text"
                            variant="filled"                          
                            value={editText}
                            onChange={(e) => setEditText(e.target.value)}
                            fullWidth
                        />
                    </Grid>
                    <Grid item xs = {9} />
                    <Grid item xs = {3}>
                        <Button
                            fullWidth
                            onClick={addTodoHandler}
                        >
                            add todo
                         </Button>
                    </Grid>
                </Grid>
            </Box>
        </Paper>
    )
}

export default TodoForm