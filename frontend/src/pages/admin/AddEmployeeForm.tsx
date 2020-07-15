import React from "react";
import { Input, Button } from "@material-ui/core";

interface Props {
  callback: () => void;
}

export default function AddEmployeeForm(props: Props) {
  let [name, setName] = React.useState("");

  const handleSubmit = () => {
    fetch("/add_employee", { method: "POST", body: JSON.stringify({ name }) })
      .then(props.callback)
      .catch((err) => {
        console.error(err);
        alert("error");
      });
  };

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        handleSubmit();
      }}
    >
      <Input
        value={name}
        placeholder="Employee Name"
        onChange={(e) => setName(e.target.value)}
        required
      />
      <Button variant="contained" type="submit">
        Add Employee
      </Button>
    </form>
  );
}
