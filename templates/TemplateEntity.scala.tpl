package entity

import play.api.libs.functional.syntax._
import play.api.libs.json.Reads._
import play.api.libs.json._

{% set i = 0 -%}
{% set suffix = "," -%}
final case class {{ EntityName }} (
{%for column in column_list -%}
  {% set i = i + 1 -%}
  {%if i >= column_list_length -%}
  {% set suffix = "" -%}
  {%endif -%}
  {%if column.is_nullable == "YES" -%}
    {{ column.column_name_camel }}: Option[{{ column.data_type }}]{{ suffix }}
  {%else -%}
    {{ column.column_name_camel }}: {{ column.data_type }}{{ suffix }}
  {%endif -%}
{% endfor -%}) extends Entity

object {{ EntityName }} {
  implicit def {{ camelCaseName }}Reads: Reads[{{ EntityName }}] = (
  {% set i = 0 -%}
  {% set suffix = " and" -%}
  {%for column in column_list -%}
    {% set i = i + 1 -%}
    {%if i >= column_list_length -%}
        {% set suffix = "" -%}
    {%endif -%}
    {%if column.is_nullable == "YES" -%}
        (JsPath \ "{{column.column_name}}").readNullable[{{column.data_type}}] {{ suffix }}
    {%else -%}
        (JsPath \ "{{column.column_name}}").read[{{column.data_type}}] {{ suffix }}
    {%endif -%}
  {% endfor -%}
  )({{ EntityName }}.apply _)

  implicit def {{ camelCaseName }}Writes: Writes[{{ EntityName }}] = (
  {% set i = 0 -%}
  {% set suffix = " and" -%}
  {%for column in column_list -%}
    {% set i = i + 1 -%}
    {%if i >= column_list_length -%}
        {% set suffix = "" -%}
    {%endif -%}
    {%if column.is_nullable == "YES" -%}
        (JsPath \ "{{column.column_name}}").writeNullable[{{column.data_type}}] {{ suffix }}
    {%else -%}
        (JsPath \ "{{column.column_name}}").write[{{column.data_type}}] {{ suffix }}
    {%endif -%}
  {% endfor -%}
  )(unlift({{ EntityName }}.unapply))

  {% set i = 0 -%}
  {% set suffix = "," -%}
  implicit def convertFromModel(model: {{ ModelName }}): {{ EntityName }} = {
    {{ EntityName }} (
      {%for column in column_list -%}
         {% set i = i + 1 -%}
         {%if i >= column_list_length -%}
         {% set suffix = "" -%}
         {%endif -%}
         model.{{ column.column_name_camel }}{{ suffix }}
      {% endfor -%}
    )
  }

  implicit def convertFromModels(models: List[{{ ModelName }}]): List[{{ EntityName }}] =
    models.map(convertFromModel)
}

