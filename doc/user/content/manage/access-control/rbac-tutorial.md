---
title: "Tutorial: Manage privileges in a non-production cluster"
description: "Add users, create roles, and assign privileges in Materialize"
menu:
  main:
    parent: access-control
    weight: 25
---

This tutorial walks you through creating a new user and managing roles in Materialize. By
the end of this tutorial you will:

* Invite a new user in the Materialize Console
* Create two new roles in your Materialize
* Apply privileges to the new roles
* Assign a role to the new user
* Modify and remove privileges on roles

In this scenario, you are an administrator on your Materialize account. You
recently hired a new developer who needs privileges in a non-production cluster.
You will create specific privileges for the new role that align with your
business needs and restrict the developer role from having access to your
production cluster.

## Before you begin

* Make sure you have a [Materialize account](https://materialize.com/register/?utm_campaign=General&utm_source=documentation) and already have a password to connect with.

## Step 1. Invite a new user

1. [Login to the Materialize console](https://console.materialize.com/) and navigate to Account > Account
Settings > Users.

1. Click **Invite User** and fill in the user information.

The **Organization Admin** and **Organization Member** roles refer to
Materialize console privileges. Select **Organization Member** for this
example.

## Step 2. Create a new role

1. In the [SQL Shell](https://console.materialize.com/), or your preferred SQL
   client connected to Materialize, create a new role:

    ```mzsql
    CREATE ROLE dev_role;
    ```

1. Each role you create has default **role attributes** that determine how they
    can interact with Materialize objects. Let's look at the role attributes of
    the role you created:

    ```mzsql
    SELECT * FROM mz_roles WHERE name = 'dev_role';
   ```

    Your `dev_role` returns attributes similar to the following:

    ```nofmt
    -[ RECORD 1 ]--+------
    id             | u8
    oid            | 50991
    name           | dev_role
    inherit        | t
    create_role    | f
    create_db      | f
    create_cluster | f
    ```
    Your `id` and `oid` values will look different.

  The `inherit` is the role attribute assigned to a role when it is created.

  * `INHERIT` is set to true by default and allows roles to inherit the
    privileges of roles it is a member of. It is not possible to set this to false.

  * `CREATEROLE` is deprecated and will be removed soon. It has no effect.

  * `CREATEDB` is deprecated and will be removed soon. It has no effect.

  * `CREATECLUSTER` is deprecated and will be removed soon. It has no effect.

## Step 3. Create example objects

Your `dev_role` has the default system-level permissions and needs object-level privileges. RBAC allows you to apply granular privileges to objects in the SQL hierarchy. Let's create some example objects in the system and determine what
privileges the role needs.

1. In the SQL client connected to Materialize, create a new example cluster to
avoid impacting other environments:

   ```mzsql
   CREATE CLUSTER dev_cluster (SIZE = '25cc');
   ```

1. Change into the example cluster:

   ```mzsql
   SET CLUSTER TO dev_cluster;
   ```

1. Create a new database, schema, and table:

   ```mzsql
   CREATE DATABASE dev_db;
   ```

   ```mzsql
   CREATE SCHEMA dev_db.schema;
   ```

   ```mzsql
   CREATE TABLE dev_table (a int, b text NOT NULL);
   ```

You just created a set of objects. Your schema object belongs to
the database. You can access the cluster from any database. The next
step is to grant privileges to your role based on the role needs.

## Step 4. Grant privileges to the role

In this example, let's say your `dev_role` needs the following permissions:

* Read, write, and append privileges on the table
* Usage privileges on the schema
* All available privileges on the database
* Usage and create privileges on the cluster

1. In your terminal, grant table-level privileges to the `dev_role`:

   ```mzsql
   GRANT SELECT, UPDATE, INSERT ON dev_table TO dev_role;
   ```

   Table objects have four available privileges - `read`, `write`, `append`, and
   `delete`. The `dev_role` doesn't need `delete` permissions, so it is not
   applied in the `GRANT` statement above.

2. Grant schema privileges to the `dev_role`:

   ```mzsql
   GRANT USAGE ON SCHEMA dev_db.schema TO dev_role;
   ```

   Schemas have `USAGE` and `CREATE` privileges available to grant.

3. Grant database privileges to the `dev_role`. You can use the `GRANT ALL`
   statement to grant all available privileges on an object.

   ```mzsql
   GRANT ALL ON DATABASE dev_db TO dev_role;
   ```

4. Grant cluster privileges to the `dev_role`:

   ```mzsql
   GRANT USAGE, CREATE ON CLUSTER dev_cluster TO dev_role;
   ```

   Materialize cluster privileges are unique to the Materialize RBAC structure.
   To have access to the objects within a cluster, you must also have the same
   level of access to the cluster itself.

## Step 5. Assign the role to a user

The `dev_role` now has the acceptable privileges it needs. Let's apply this role
to a user in your Materialize organization.

1. In your terminal, use the `GRANT` statement to apply a role to your new user:

   ```mzsql
   GRANT dev_role TO <new_user>;
   ```

1. To review the permissions a role has, you can view the object data:

   ```mzsql
   SELECT name, privileges FROM mz_tables WHERE name='dev_table';
   ```

   The output should return the object ID, the level of permission, and the assigning role ID.

   ```nofmt
   name|privileges
   dev_table|{u1=arwd/u1,u8=arw/u1}
   (1 row)
   ```

   In this example, role ID `u1` has append, read, write, and delete
   privileges on the table. Object ID `u8` is the `dev_role` and has append, read, and write privileges,
   which were assigned by the `u1` user.

## Step 6. Create a second role

Next, you will create a new role with different privileges to other objects.
Then you will apply those privileges to the `dev` role and alter or drop
privileges as needed.

1. Create a second role your Materialize account:

   ```mzsql
   CREATE ROLE qa_role;
   ```

2. Apply `CREATEDB` privileges to the `qa_role`

   ```mzsql
   GRANT CREATEDB ON SYSTEM TO qa_role;
   ```

   This role has permission to create a new database in the Materialize account.

3. Create a new `qa_db` database:

   ```mzsql
   CREATE DATABASE qa_db;
   ```

4. Apply `USAGE` and `CREATE` privileges to the `qa_role` role for the new database:

   ```mzsql
   GRANT USAGE, CREATE ON DATABASE qa_db TO qa_role;
   ```

## Step 7. Add inherited privileges

Your `dev_role` also needs access to `qa_db`. You can apply these
privileges individually or you can choose to grant the `dev_role` the same
permissions as the `qa_role`.

1. Add `dev_role` as a member of `qa_role`:

   ```mzsql
   GRANT qa_role TO dev_role;
   ```

   Roles also inherit all the privileges of the granted role.
   Making roles members of other roles allows you to manage sets of
   permissions, rather than granting privileges to roles on an individual basis.

2. Review the privileges of `qa_role` and `dev_role`:

   ```mzsql
   SELECT name, privileges FROM mz_databases WHERE name='qa_db';
   ```

   Your output will be similar to the example below:

   ```nofmt
   name|privileges
   qa_db|{u1=UC/u1,u9=UC/u1}
   (1 row)
   ```

   Both `dev_role` and `qa_role` have usage and create access to the `qa_db`. In
   the next section, you will edit role attributes for these roles and drop
   privileges.

## Step 8. Revoke privileges

You can revoke certain privileges for each role, even if they are inherited from another role.

1. Let's say you decide `dev_role` no longer needs `CREATE` privileges on the
   `dev_table` object. You can revoke that privilege for the role:

   ```mzsql
   REVOKE CREATE ON DATABASE dev_table FROM dev_role;
   ```

   Your output should contain the new privileges for `dev_role`:

   ```nofmt
   name|privileges
   qa_db|{u1=UC/u1,u8=U/u1,u9=UC/u1}
   (1 row)
   ```

   {{< note >}}
   If you need to revoke specific privileges from a role that have been
   inheritied from another role, you must revoke the role with those privileges.

   ```mzsql
   REVOKE qa_role FROM dev_role;
   ```
   In this example, when `dev_role` inherits from `qa_role`, `dev_role` always has
   **all** privileges of `qa_role`. You cannot revoke specific privileges for an
   inherited role because inheritance gives effective permissions for the
   entire role.

   {{</ note >}}

## Next steps

You just altered privileges and attributes on your Materialize roles! Remember
to destroy the objects you created for this guide.

1. Drop the roles you created:

   ```mzsql
   DROP ROLE qa_role;
   DROP ROLE dev_role;
   ```

1. Drop the other objects you created:

   ```mzsql
   DROP CLUSTER dev_cluster CASCADE;
   DROP DATABASE dev_db CASCADE;
   DROP TABLE dev_table;
   DROP DATABASE qa_db CASCADE;
   ```
## Related pages

For more information on RBAC in Materialize, review the reference documentation:

* [`GRANT ROLE`](/sql/grant-role/)
* [`CREATE ROLE`](/sql/create-role/)
* [`GRANT PRIVILEGE`](/sql/grant-privilege/)
* [`ALTER ROLE`](/sql/alter-role/)
* [`REVOKE PRIVILEGE`](/sql/revoke-privilege/)
* [`DROP ROLE`](/sql/drop-role/)
