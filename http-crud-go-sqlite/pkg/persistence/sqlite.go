package persistence

import (
	"context"
	"fmt"

	"github.com/fermyon/enterprise-architectures-and-patterns/http-crud-go-sqlite/pkg/types"
	"github.com/fermyon/spin/sdk/go/v2/sqlite"
	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
)

const (
	db_name                   = "default"
	sqlx_db_driver            = "sqlite"
	sql_read_all              = "SELECT ID,NAME,case when ACTIVE = 1 then 'true' else 'false' end as ACTIVE FROM ITEMS"
	sql_read_by_id            = "SELECT ID,NAME,case when ACTIVE = 1 then 'true' else 'false' end as ACTIVE FROM ITEMS WHERE Id = ? LIMIT 1;"
	sql_create_item           = "INSERT INTO ITEMS (ID,NAME,ACTIVE) VALUES (?,?,?)"
	sql_update_item_by_id     = "UPDATE ITEMS SET NAME=?, ACTIVE=? WHERE ID=?"
	sql_delete_item_by_id     = "DELETE FROM ITEMS WHERE ID=?"
	sql_delete_multiple_items = "DELETE FROM ITEMS WHERE ID IN ("
)

func db() *sqlx.DB {
	conn := sqlite.Open(db_name)
	return sqlx.NewDb(conn, sqlx_db_driver)
}

func CreateItem(ctx context.Context, model types.ItemCreateModel) (*types.Item, error) {
	id, err := uuid.NewRandom()
	if err != nil {
		return nil, fmt.Errorf("failed to create identifier for new item %v", err)
	}
	item := model.NewItemWithId(id.String())
	connection := db()
	if connection == nil {
		return nil, fmt.Errorf("failed to establish database connection")
	}
	defer connection.Close()
	_, err = connection.Query(sql_create_item, item.Id, item.Name, toInt(item.Active))
	if err != nil {
		return nil, fmt.Errorf("failed to store item in database %v", err)
	}

	return &item, nil

}

func UpdateItemById(ctx context.Context, id string, model types.ItemUpdateModel) (*types.Item, error) {
	item := model.AsItem(id)
	connection := db()
	if connection == nil {
		return nil, fmt.Errorf("failed to establish database connection")
	}
	defer connection.Close()
	_, err := connection.ExecContext(ctx, sql_update_item_by_id, item.Name, toInt(item.Active), item.Id)
	if err != nil {
		return nil, fmt.Errorf("failed to update item in database %v", err)
	}
	return &item, nil
}

func DeleteMultipleItems(ctx context.Context, model types.BatchDeleteModel) error {
	if len(model.Ids) == 0 {
		return nil
	}
	command := sql_delete_multiple_items
	for i := 0; i < len(model.Ids); i++ {
		command = fmt.Sprintf("%s?", command)
		if i < len(model.Ids)-1 {
			command = fmt.Sprintf("%s,", command)
		}
	}
	command = fmt.Sprintf("%s)", command)
	connection := db()
	if connection == nil {
		return fmt.Errorf("failed to establish database connection")
	}
	defer connection.Close()

	parameters := make([]interface{}, len(model.Ids))
	for i, v := range model.Ids {
		parameters[i] = v
	}
	_, err := connection.ExecContext(ctx, command, parameters...)
	if err != nil {
		return err
	}
	return nil
}

func DeleteItemById(ctx context.Context, id string) error {
	connection := db()
	if connection == nil {
		return fmt.Errorf("failed to establish database connection")
	}
	defer connection.Close()
	_, err := connection.ExecContext(ctx, sql_delete_item_by_id, id)
	if err != nil {
		return err
	}
	return nil
}

func ReadItemById(ctx context.Context, id string) (*types.Item, error) {
	connection := db()
	if connection == nil {
		return nil, fmt.Errorf("failed to establish database connection")
	}
	defer connection.Close()

	rows, err := connection.Queryx(sql_read_by_id, id)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	for rows.Next() {
		var item types.Item
		err = rows.StructScan(&item)
		if err != nil {
			return nil, err
		}

		return &item, nil
	}

	return nil, nil
}

func ReadAllItems(ctx context.Context) ([]*types.Item, error) {
	connection := db()
	if connection == nil {
		return nil, fmt.Errorf("failed to establish database connection")
	}
	defer connection.Close()

	rows, err := connection.Queryx(sql_read_all)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	items := []*types.Item{}
	for rows.Next() {
		var item types.Item
		err = rows.StructScan(&item)
		if err != nil {
			return nil, err
		}
		items = append(items, &item)
	}
	return items, nil
}

func toInt(value bool) int {
	if value {
		return 1
	}
	return 0
}
