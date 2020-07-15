import React from "react";
import { Button } from "@material-ui/core";

interface Props {
  employee_id: number;
  callback: () => void;
}

export default function DeleteEmployeeButton(props: Props) {
  const handleDelete = () => {
    if (!window.confirm("Are you sure you wish to remove this employee?")) {
      return;
    }

    fetch("delete_employee", {
      method: "POST",
      body: JSON.stringify({ employee_id: props.employee_id }),
    }).then(props.callback);
  };

  return (
    <Button variant="contained" color="secondary" onClick={handleDelete}>
      Delete
    </Button>
  );
}
