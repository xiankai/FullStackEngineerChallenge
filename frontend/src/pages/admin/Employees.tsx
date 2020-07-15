import React from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
} from "@material-ui/core";
import { Link } from "react-router-dom";
import AddEmployeeForm from "./AddEmployeeForm";
import DeleteEmployeeButton from "./DeleteEmployeeButton";

interface Employee {
  id: number;
  name: string;
}

export default function Employees() {
  const [employees, setEmployees] = React.useState<Employee[]>([]);

  const fetchEmployees = () => {
    fetch("get_employees")
      .then((res) => res.json())
      .then((json) => setEmployees(json));
  };

  // Initial load
  React.useEffect(() => {
    fetchEmployees();
  }, []);

  return (
    <>
      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Employee ID</TableCell>
              <TableCell align="right">Name</TableCell>
              <TableCell align="right">&nbsp;</TableCell>
              <TableCell align="right">&nbsp;</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {employees.map((employee) => (
              <TableRow key={employee.id}>
                <TableCell component="th" scope="row">
                  {employee.id}
                </TableCell>
                <TableCell align="right">{employee.name}</TableCell>
                <TableCell align="right">
                  <Link to={`/employee/${employee.id}`}>Reviews</Link>
                </TableCell>
                <TableCell align="right">
                  <DeleteEmployeeButton
                    employee_id={employee.id}
                    callback={fetchEmployees}
                  />
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
      <AddEmployeeForm callback={fetchEmployees} />
    </>
  );
}
