import React from "react";
import Employees from "./pages/admin/Employees";
import Reviews from "./pages/admin/Reviews";
import { BrowserRouter, Switch, Route } from "react-router-dom";

function App() {
  return (
    <BrowserRouter>
      <Switch>
        <Route path="/employee/:employee_id" component={Reviews} />
        <Route path="/" component={Employees} />
      </Switch>
    </BrowserRouter>
  );
}

export default App;
