package models

import "gorm.io/gorm"

type User struct {
	gorm.Model
	Name   string `json:"name" gorm:"not null;size:100"`
	Role   string `json:"role" gorm:"not null;size:20"` // Educator, Student, Designer
	Email  string `json:"email" gorm:"uniqueIndex;not null;size:100"`
	Tokens int    `json:"tokens" gorm:"default:0"`
}

// TableName specifies the table name for User model
func (User) TableName() string {
	return "users"
}